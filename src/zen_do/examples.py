from typing import Optional

from zen_do.zenodo_client import ZenodoDepositState, ZenodoResponse
from zen_do.zenodo_metadata import (
    ZenodoCreator,
    ZenodoMetadata,
    ZenodoRelatedIdentifier,
)


def example_metadata(title: str = "Test Book") -> ZenodoMetadata:
    """A set of example Zenodo metadata."""
    return ZenodoMetadata(
        title=title,
        upload_type="poster",
        creators=[
            ZenodoCreator(
                name="Doe, Jane", affiliation="University of Testfalia", orcid="ABC"
            )
        ],
        related_identifiers=[
            ZenodoRelatedIdentifier(
                identifier="urn:zenodo:my-org:project:book",
                relation="isIdenticalTo",
                resource_type="other",
                scheme="urn",
            )
        ],
    )


def example_deposit(
    id: int = 123,
    metadata: ZenodoMetadata = example_metadata(),
    state: ZenodoDepositState = ZenodoDepositState.done,
    submitted: bool = True,
    bucket: Optional[str] = "https://path.com/path/wrwee-324-23f-sdf",
    urn: str = "urn:zenodo:my-org:project:book",
) -> ZenodoResponse:
    """An example Zenodo deposit."""
    metadata = metadata.model_copy(
        update={
            "related_identifiers": [
                ZenodoRelatedIdentifier(
                    identifier=urn,
                    relation="isIdenticalTo",
                    resource_type="other",
                    scheme="urn",
                )
            ]
        }
    )
    return {
        "id": id,
        "metadata": metadata.model_dump(),
        "state": state,
        "submitted": submitted,
        "links": {"bucket": bucket} if bucket else {},
    }
