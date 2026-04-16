import os

import keyring
from keyring.errors import NoKeyringError

SERVICE_NAME = "zen-do"


def get_token(sandbox: bool = False) -> str:
    """Gets the Zenodo token from the system keyring or environment variables.

    Args:
        sandbox: Whether to get the token for the Zenodo sandbox environment.

    Returns:
        The token.

    Raises:
        RuntimeError: If no token is found.
    """
    token_name = "ZENODO_SANDBOX_TOKEN" if sandbox else "ZENODO_TOKEN"
    try:
        token = keyring.get_password(service_name=SERVICE_NAME, username=token_name)
    except NoKeyringError:
        token = None

    token = token or os.getenv(token_name)

    if not token:
        raise RuntimeError(
            f"No Zenodo access token found for {token_name!r} in the system "
            "keyring or the environment variables. You need to add it in order "
            "to access the Zenodo API."
        )
    return token
