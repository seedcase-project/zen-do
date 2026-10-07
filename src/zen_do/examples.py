from zen_do.zenodo_client import (
    ZenodoDepositState,
    ZenodoResponse,
)
from zen_do.zenodo_metadata import (
    ZenodoMetadata,
    ZenodoRelatedIdentifier,
)


def example_deposit(
    id: int = 123,
    metadata: ZenodoMetadata | None = None,
    state: ZenodoDepositState = ZenodoDepositState.done,
    submitted: bool = True,
    bucket: str | None = "https://path.com/path/wrwee-324-23f-sdf",
    urn: str = "urn:zenodo:my-org:project:book",
) -> ZenodoResponse:
    """An example Zenodo deposit."""
    if metadata is None:
        metadata = example_metadata()

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
