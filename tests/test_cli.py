from unittest.mock import patch

from pytest import fixture, raises

from zen_do.cli import app


@fixture
def _mock_zenodo_get_deposit(mocker):
    return mocker.patch("zen_do.cli.zenodo_get_deposit")


@fixture
def _mock_client():
    with patch("zen_do.cli.ZenodoClient") as mocked_client:
        mocked_client.return_value.get_deposits.return_value = []
        yield mocked_client


def test_zenodo_publish_existing_deposit(
    monkeypatch,
    _mock_zenodo_get_deposit,
    _mock_client,
):
    monkeypatch.setenv("ZENODO_TOKEN", "token")
    app("zenodo-publish", result_action="return_value")


def test_zenodo_publish_new_deposit(
    monkeypatch,
    _mock_zenodo_get_deposit,
    _mock_client,
):
    monkeypatch.setenv("ZENODO_TOKEN", "token")
    _mock_zenodo_get_deposit.return_value = None
    app("zenodo-publish", result_action="return_value")


def test_zenodo_publish_needs_token():
    with raises(RuntimeError):
        app("zenodo-publish", result_action="return_value")
