import os

import keyring
from cyclopts import App

from zen_do.zenodo import (
    zenodo_create_record,
    zenodo_get_record,
    zenodo_update_record,
)

app = App(
    name="zen-do",
    help=(
        "The `zen_do` package contains GitHub reusable workflows and "
        "actions used in the Seedcase Project."
    ),
)


@app.command()
def zenodo_publish(sandbox: bool = False) -> None:
    """Publish a new version of the repository on Zenodo."""
    token = get_token(sandbox)
    if record := zenodo_get_record(token):
        zenodo_update_record(token, record.id)
        print("Zenodo record updated successfully!")
    else:
        zenodo_create_record(token)
        print("New Zenodo record created successfully!")


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
    token = keyring.get_password("system", token_name) or os.getenv(token_name)
    if not token:
        raise RuntimeError(
            f"No value found for {token_name!r} in the system "
            "keyring or the environment variables."
        )
    return token
