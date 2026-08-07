from pytest import fixture

from zen_do.cli import app
from zen_do.examples import example_deposit, example_metadata
from zen_do.internals import _read_metadata, _write_metadata


@fixture
def _mock_zenodo_get_deposit(mocker):
    return mocker.patch("zen_do.cli.zenodo_get_deposit")


@fixture
def _mock_client(mocker, monkeypatch):
    monkeypatch.setenv("ZENODO_TOKEN", "token")
    return mocker.patch("zen_do.cli.ZenodoClient")


def test_list_when_deposits_found(
    capsys,
    monkeypatch,
    _mock_client,
):
    monkeypatch.setenv("ZENODO_TOKEN", "token")
    deposit = example_deposit()
    _mock_client.return_value.get_deposits.return_value = [deposit]

    app("list", result_action="return_value")
    out = capsys.readouterr().out

    assert str(deposit["id"]) in out


def test_list_when_no_deposits_found(
    capsys,
    monkeypatch,
    _mock_client,
):
    monkeypatch.setenv("ZENODO_TOKEN", "token")
    _mock_client.return_value.get_deposits.return_value = []

    app("list", result_action="return_value")
    out = capsys.readouterr().out

    assert "[" not in out


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
