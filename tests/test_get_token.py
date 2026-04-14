from pytest import mark, raises

from zen_do.cli import get_token


@mark.parametrize("sandbox", [False, True])
def test_get_token_flags_no_token_found(sandbox):
    with raises(RuntimeError):
        get_token(sandbox)


@mark.parametrize(
    "sandbox, token_name", [(False, "ZENODO_TOKEN"), (True, "ZENODO_SANDBOX_TOKEN")]
)
def test_gets_token_from_keyring(monkeypatch, mocker, sandbox, token_name):
    monkeypatch.setenv(token_name, f"{token_name}_env_var")
    mock = mocker.patch("keyring.get_password", return_value=f"{token_name}_keyring")

    assert get_token(sandbox) == f"{token_name}_keyring"
    mock.assert_called_once_with(service_name="zen-do", username=token_name)


@mark.parametrize(
    "sandbox, token_name", [(False, "ZENODO_TOKEN"), (True, "ZENODO_SANDBOX_TOKEN")]
)
def test_gets_token_from_env_vars_if_not_in_keyring(monkeypatch, sandbox, token_name):
    monkeypatch.setenv("ZENODO_TOKEN", "ZENODO_TOKEN_value")
    monkeypatch.setenv("ZENODO_SANDBOX_TOKEN", "ZENODO_SANDBOX_TOKEN_value")

    assert get_token(sandbox) == f"{token_name}_value"
