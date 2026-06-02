from pytest import fixture, raises

from zen_do.cli import app
from zen_do.examples import example_deposit


@fixture
def _mock_zenodo_get_deposit(mocker):
    return mocker.patch("zen_do.cli.zenodo_get_deposit")


@fixture
def _mock_client(mocker):
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


def test_zenodo_publish_needs_token():
    with raises(RuntimeError):
        app("list", result_action="return_value")
