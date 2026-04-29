from pathlib import Path

import seedcase_soil as so
from rich import print_json
from seedcase_soil import (
    # print_if_verbose,
    run_without_tracebacks,
    setup_cli,
)

from zen_do.get_token import get_token
from zen_do.zenodo_client import ZenodoClient
from zen_do.zenodo_get_deposit import zenodo_get_deposit

app = setup_cli(
    name="zen-do",
    help="zen-do simplifies interacting with Zenodo for common publishing tasks.",
)


@app.command()
def zenodo_publish(sandbox: bool = False) -> None:
    """Publish a new version of the repository on Zenodo."""
    token = get_token(sandbox)
    client = ZenodoClient(token, sandbox)
    if deposit := zenodo_get_deposit(client.get_deposits()):
        print("Zenodo deposit updated successfully!")
        print(f"{deposit}")
    else:
        print("New Zenodo record created successfully!")


@app.command()
def get(
    metadata_file: Path = Path(".zenodo.toml"), /, *, sandbox: bool = False
) -> None:
    """Get the Zenodo deposit JSON based on the metadata file.

    Args:
        metadata_file: The path to the metadata file.
        sandbox: Whether to use the Zenodo sandbox environment for testing purposes.
    """
    token = get_token(sandbox)
    client = ZenodoClient(token, sandbox)
    deposit = zenodo_get_deposit(client.get_deposits(), metadata_file)

    if deposit:
        print_json(data=deposit)
    else:
        so.pretty_print(
            f"No deposit found on Zenodo for metadata file '{metadata_file}'."
        )


def main() -> None:
    """Create an entry point to run the cli without tracebacks."""
    run_without_tracebacks(app)
