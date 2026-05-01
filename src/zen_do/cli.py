from pathlib import Path

import seedcase_soil as so
from seedcase_soil import (
    # print_if_verbose,
    run_without_tracebacks,
    setup_cli,
)

from zen_do.get_token import get_token
from zen_do.internals import _write_metadata
from zen_do.zenodo_client import ZenodoClient
from zen_do.zenodo_get_deposit import zenodo_get_deposit
from zen_do.zenodo_metadata import (
    ZenodoCreator,
    ZenodoMetadata,
    ZenodoRelatedIdentifier,
)

app = setup_cli(
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
def zenodo_publish(sandbox: bool = False) -> None:
    """Publish a new version of the repository on Zenodo."""
    token = get_token(sandbox)
    client = ZenodoClient(token, sandbox)
    if deposit := zenodo_get_deposit(client.get_deposits()):
        print("Zenodo deposit updated successfully!")
        print(f"{deposit}")
    else:
        print("New Zenodo record created successfully!")


def main() -> None:
    """Create an entry point to run the cli without tracebacks."""
    run_without_tracebacks(app)
