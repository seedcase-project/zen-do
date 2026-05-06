from pathlib import Path

import seedcase_soil as so
from cyclopts import Parameter
from seedcase_soil import (
    # print_if_verbose,
    run_without_tracebacks,
    setup_cli,
)
from typing_extensions import Annotated

from zen_do.convert import Format
from zen_do.get_token import get_token
from zen_do.internals import _read_metadata
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
def convert(
    metadata_path: Path = Path(".zenodo.toml"),
    /,
    *,
    to: Annotated[list[Format], Parameter(consume_multiple=True)],
    verbose: bool = False,
) -> None:
    """Convert a metadata file, e.g. `.zenodo.toml`, to other formats.

    Args:
        metadata_path: The path to the metadata file to convert.
        to: The formats to convert the metadata file to. Can be a single format
            or an array of formats.
        verbose: Whether to print a log of the actions done.
    """
    metadata = _read_metadata(metadata_path)
    converted_files = so.fmap(
        to, lambda format: format.convert(metadata, metadata_path)
    )
    for file in converted_files:
        file.output_path.write_text(file.content)
        so.print_if_verbose(
            verbose, f"Metadata file created or updated at '{file.output_path}'."
        )


def main() -> None:
    """Create an entry point to run the cli without tracebacks."""
    run_without_tracebacks(app)
