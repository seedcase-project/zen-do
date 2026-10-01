import re
import subprocess
from datetime import UTC, datetime
from pathlib import Path

from zen_do.get_token import get_token
from zen_do.internals import _write_metadata
from zen_do.zenodo_client import ZenodoClient, ZenodoResponse
from zen_do.zenodo_metadata import (
    ZenodoCreator,
    ZenodoMetadata,
    ZenodoRelatedIdentifier,
)


def test_publish(monkeypatch, tmp_path):
    """Integration test for the `zen-do publish` command.

    First, a new Zenodo deposit is created and published with a file and metadata.
    Then, the file and metadata are updated, and the existing deposit is updated and
    published with the new information.

    As it is not possible to delete a published Zenodo deposit, each test run creates a
    new deposit with a unique timestamp in the title and related identifier.
    """
    monkeypatch.chdir(tmp_path)
    timestamp = datetime.now(UTC).strftime("%Y-%m-%dT%H-%M-%SZ")

    # CREATE

    ## Create file to publish
    file_path = Path("test.txt")
    file_path.write_text("This is a test file.")

    ## Create Zenodo metadata file
    metadata = ZenodoMetadata(
        title=f"zen-do integration test {timestamp}",
        upload_type="publication",
        creators=[
            ZenodoCreator(
                name="John Doe",
                affiliation="Test University",
                orcid="0000-0002-1825-0097",
            )
        ],
        related_identifiers=[
            ZenodoRelatedIdentifier(
                identifier=f"urn:zenodo:test-org:zen-do:test:{timestamp}",
                relation="isIdenticalTo",
                resource_type="other",
                scheme="urn",
            )
        ],
    )
    _write_metadata(metadata)

    ## Create new Zenodo deposit using the CLI
    output = _run_cli_publish(file_path)

    ## Check command output
    assert "created successfully" in output
    deposit_id = _get_deposit_id_from_output(output)

    ## Check deposit directly on Zenodo
    client = ZenodoClient(get_token(sandbox=True), sandbox=True)
    deposit = _check_deposit_on_zenodo(deposit_id, metadata, file_path, client)
    file_size = deposit["files"][0]["filesize"]

    # UPDATE

    ## Update file and metadata for the next publish
    file_path.write_text("This is an updated test file.")

    updated_metadata = metadata.model_copy(
        update={"title": metadata.title + " (updated)"}
    )
    _write_metadata(updated_metadata)

    ## Update existing Zenodo deposit using the CLI
    output = _run_cli_publish(file_path)

    ## Check command output
    assert "updated successfully" in output
    deposit_id = _get_deposit_id_from_output(output)

    ## Check deposit directly on Zenodo
    deposit = _check_deposit_on_zenodo(deposit_id, updated_metadata, file_path, client)
    updated_file_size = deposit["files"][0]["filesize"]
    assert updated_file_size > file_size, "New file should be larger than old file."


def _run_cli_publish(file_path: Path) -> str:
    return subprocess.run(
        [
            "uv",
            "run",
            "zen-do",
            "publish",
            "--file-path",
            str(file_path),
            "--sandbox",
            "--verbose",
        ],
        check=True,
        capture_output=True,
        text=True,
    ).stdout


def _get_deposit_id_from_output(output: str) -> str:
    id_match = re.search(r'"id": (\d+),', output)
    assert id_match is not None
    return id_match.group(1)


def _check_deposit_on_zenodo(
    deposit_id: str, metadata: ZenodoMetadata, file_path: Path, client: ZenodoClient
) -> ZenodoResponse:
    deposit = client.get_deposit(deposit_id)

    # Check metadata
    assert deposit["metadata"]["title"] == metadata.title
    assert (
        deposit["metadata"]["related_identifiers"][0]
        == metadata.related_identifiers[0].model_dump()
    )
    assert deposit["metadata"]["creators"][0] == metadata.creators[0].model_dump()

    # Check files
    assert len(deposit["files"]) == 1
    assert deposit["files"][0]["filename"] == file_path.name

    return deposit
