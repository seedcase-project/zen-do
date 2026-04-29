from pytest import fixture, raises

from zen_do.cli import app
from zen_do.examples import example_deposit


@fixture
def _mock_zenodo_get_deposit(mocker):
    return mocker.patch("zen_do.cli.zenodo_get_deposit")


@fixture
def _mock_client(mocker, monkeypatch):
    monkeypatch.setenv("ZENODO_TOKEN", "token")
    return mocker.patch("zen_do.cli.ZenodoClient")


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


def test_get_when_deposit_found(
    capsys,
    _mock_client,
    _mock_zenodo_get_deposit,
):
    deposit = example_deposit()
    _mock_zenodo_get_deposit.return_value = deposit

    app("get", result_action="return_value")
    out = capsys.readouterr().out

    assert str(deposit["id"]) in out


def test_get_when_deposit_not_found(
    capsys,
    _mock_client,
    _mock_zenodo_get_deposit,
):
    _mock_zenodo_get_deposit.return_value = None

    app("get", result_action="return_value")
    out = capsys.readouterr().out

    assert "{" not in out
