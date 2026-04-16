from pytest import fixture, raises

from zen_do.cli import app


@fixture
def _mock_zenodo_get_deposit(mocker):
    return mocker.patch("zen_do.cli.zenodo_get_deposit")


@fixture
def _mock_zenodo_update_deposit(mocker):
    return mocker.patch("zen_do.cli.zenodo_update_deposit")


@fixture
def _mock_zenodo_create_deposit(mocker):
    return mocker.patch("zen_do.cli.zenodo_create_deposit")


def test_zenodo_publish_existing_deposit(
    monkeypatch,
    _mock_zenodo_get_deposit,
    _mock_zenodo_create_deposit,
    _mock_zenodo_update_deposit,
):
    monkeypatch.setenv("ZENODO_TOKEN", "token")
    app("zenodo-publish", result_action="return_value")

    _mock_zenodo_update_deposit.assert_called_once()
    _mock_zenodo_create_deposit.assert_not_called()


def test_zenodo_publish_new_deposit(
    monkeypatch,
    _mock_zenodo_get_deposit,
    _mock_zenodo_create_deposit,
    _mock_zenodo_update_deposit,
):
    monkeypatch.setenv("ZENODO_TOKEN", "token")
    _mock_zenodo_get_deposit.return_value = None
    app("zenodo-publish", result_action="return_value")

    _mock_zenodo_create_deposit.assert_called_once()
    _mock_zenodo_update_deposit.assert_not_called()


def test_zenodo_publish_needs_token():
    with raises(RuntimeError):
        app("zenodo-publish", result_action="return_value")
