from keyring.errors import NoKeyringError
from pytest import mark, raises

from zen_do.get_token import SERVICE_NAME, get_token


@mark.parametrize("sandbox", [False, True])
def test_get_token_flags_no_token_found(sandbox, mocker):
    mocker.patch("keyring.get_password", return_value=None)
    with raises(RuntimeError):
        get_token(sandbox)


@mark.parametrize(
    "sandbox, token_name", [(False, "ZENODO_TOKEN"), (True, "ZENODO_SANDBOX_TOKEN")]
)
def test_gets_token_from_keyring(monkeypatch, mocker, sandbox, token_name):
    monkeypatch.setenv(token_name, f"{token_name}_env_var")
    mock = mocker.patch("keyring.get_password", return_value=f"{token_name}_keyring")

    assert get_token(sandbox) == f"{token_name}_keyring"
    mock.assert_called_once_with(service_name=SERVICE_NAME, username=token_name)


@mark.parametrize(
    "sandbox, token_name", [(False, "ZENODO_TOKEN"), (True, "ZENODO_SANDBOX_TOKEN")]
)
def test_gets_token_from_env_vars_if_not_in_keyring(
    mocker, monkeypatch, sandbox, token_name
):
    mocker.patch("keyring.get_password", return_value=None)
    monkeypatch.setenv("ZENODO_TOKEN", "ZENODO_TOKEN_value")
    monkeypatch.setenv("ZENODO_SANDBOX_TOKEN", "ZENODO_SANDBOX_TOKEN_value")

    assert get_token(sandbox) == f"{token_name}_value"


def test_gets_token_from_env_vars_if_keyring_raises(monkeypatch, mocker):
    monkeypatch.setenv("ZENODO_TOKEN", "ZENODO_TOKEN_value")
    mocker.patch("keyring.get_password", side_effect=NoKeyringError)
    assert get_token(False) == "ZENODO_TOKEN_value"
