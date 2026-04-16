from pytest import fixture, raises

from zen_do.cli import app


@fixture
def _mock_zenodo_get_record(mocker):
    return mocker.patch("zen_do.cli.zenodo_get_record")


def test_zenodo_publish_existing_record(
    monkeypatch,
    _mock_zenodo_get_record,
):
    monkeypatch.setenv("ZENODO_TOKEN", "token")
    app("zenodo-publish", result_action="return_value")


def test_zenodo_publish_new_record(
    monkeypatch,
    _mock_zenodo_get_record,
):
    monkeypatch.setenv("ZENODO_TOKEN", "token")
    _mock_zenodo_get_record.return_value = None
    app("zenodo-publish", result_action="return_value")


def test_zenodo_publish_needs_token():
    with raises(RuntimeError):
        app("zenodo-publish", result_action="return_value")
