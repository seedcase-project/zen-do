from pytest import fixture, raises

from zen_do.cli import app
from zen_do.examples import example_metadata
from zen_do.internals import _read_metadata, _write_metadata


@fixture
def _mock_zenodo_get_deposit(mocker):
    return mocker.patch("zen_do.cli.zenodo_get_deposit")


@fixture
def _mock_client(mocker):
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


def test_init_does_not_overwrite_existing_file(monkeypatch, tmp_path):
    monkeypatch.chdir(tmp_path)
    old_metadata = example_metadata()
    _write_metadata(old_metadata)

    app("init", result_action="return_value")

    assert _read_metadata() == old_metadata


def test_init_creates_file_in_root(monkeypatch, tmp_path):
    monkeypatch.chdir(tmp_path)
    app("init", result_action="return_value")

    metadata = _read_metadata()
    assert (
        metadata.related_identifiers[0].identifier
        == f"urn:zenodo:<github-org>:{tmp_path.name}"
    )


def test_init_creates_file_in_subfolder(monkeypatch, tmp_path):
    subfolder = tmp_path / "subfolder"
    subfolder.mkdir()
    monkeypatch.chdir(subfolder)

    app("init", result_action="return_value")

    metadata = _read_metadata(subfolder / ".zenodo.toml")
    assert (
        metadata.related_identifiers[0].identifier
        == f"urn:zenodo:<github-org>:{subfolder.name}"
    )
