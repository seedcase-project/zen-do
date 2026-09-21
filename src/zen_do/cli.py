from pathlib import Path

import seedcase_soil as so
from rich import print_json

from zen_do.get_token import get_token
from zen_do.internals import _read_metadata, _write_metadata
from zen_do.zenodo_client import ZenodoClient
from zen_do.zenodo_get_deposit import zenodo_get_deposit
from zen_do.zenodo_metadata import (
    ZenodoCreator,
    ZenodoMetadata,
    ZenodoRelatedIdentifier,
)

app = so.setup_cli(
    name="zen-do",
    help="zen-do simplifies interacting with Zenodo for common publishing tasks.",
)


@app.command()
def init(verbose: bool = False) -> None:
    """Create an empty `.zenodo.toml` file that has all the metadata fields.

    Args:
        verbose: Whether to print a log of the actions done.
    """
    metadata_path = Path(".zenodo.toml")
    if metadata_path.is_file():
        so.print_if_verbose(
            verbose, "A `.zenodo.toml` file already exists in this directory."
        )
        return

    metadata = ZenodoMetadata(
        title="",
        upload_type="",
        creators=[ZenodoCreator(name="", affiliation="", orcid="")],
        related_identifiers=[
            ZenodoRelatedIdentifier(
                identifier=f"urn:zenodo:<github-org>:{Path.cwd().name}",
                relation="isIdenticalTo",
                resource_type="other",
                scheme="urn",
            )
        ],
    )
    _write_metadata(metadata, metadata_path)
    so.print_if_verbose(verbose, "Created an empty `.zenodo.toml` file.")


@app.command()
def list(sandbox: bool = False) -> None:
    """List all Zenodo deposits in an account as raw JSON (from the Zenodo servers).

    Args:
        sandbox: Whether to use the Zenodo sandbox environment for testing purposes.
    """
    token = get_token(sandbox)
    client = ZenodoClient(token, sandbox)
    deposits = client.get_deposits()

    if deposits:
        print_json(data=deposits)
    else:
        so.pretty_print("No deposits found.")


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
    metadata = _read_metadata(metadata_file)
    deposit = zenodo_get_deposit(client.get_deposits(), metadata)

    if deposit:
        print_json(data=deposit)
    else:
        so.pretty_print(
            f"No deposit found on Zenodo for metadata file '{metadata_file}'."
        )


@app.command()
def publish(
    metadata_file: Path = Path(".zenodo.toml"),
    /,
    *,
    file_path: Path | None = None,
    sandbox: bool = False,
    verbose: bool = False,
) -> None:
    """Create or update a Zenodo deposit.

    Args:
        metadata_file: The path to the metadata file.
        file_path: The path to the file to upload.
        sandbox: Whether to use the Zenodo sandbox environment for testing purposes.
        verbose: Whether to print a log of the actions done.
    """
    token = get_token(sandbox)
    client = ZenodoClient(token, sandbox)
    metadata = _read_metadata(metadata_file)
    deposit = zenodo_get_deposit(client.get_deposits(), metadata)
    success_state = "updated"

    if deposit:
        if file_path:
            deposit = client.new_version(deposit)
            client.upload_file(deposit, file_path)
        deposit = client.update_metadata(deposit, metadata)
    else:
        if not file_path:
            raise ValueError(
                "New deposits must have a file to upload. Please provide a file path "
                "using the `--file-path` option."
            )
        success_state = "created"
        deposit = client.create(metadata)
        client.upload_file(deposit, file_path)

    deposit = client.publish(deposit)

    if verbose:
        so.pretty_print(f"Zenodo deposit {success_state} successfully!")
        print_json(data=deposit)


def main() -> None:
    """Create an entry point to run the cli without tracebacks."""
    so.run_without_tracebacks(app)
