from pytest import raises

from zen_do.cli import app


def test_zenodo_publish_runs(monkeypatch):
    monkeypatch.setenv("ZENODO_TOKEN", "token")
    app("zenodo-publish", result_action="return_value")


def test_zenodo_publish_needs_token():
    with raises(RuntimeError):
        app("zenodo-publish", result_action="return_value")
