from pathlib import Path

import seedcase_soil as so
from rich import print_json

from zen_do.get_token import get_token
from zen_do.internals import _write_metadata
from zen_do.zenodo_client import ZenodoClient
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


def main() -> None:
    """Create an entry point to run the cli without tracebacks."""
    so.run_without_tracebacks(app)
