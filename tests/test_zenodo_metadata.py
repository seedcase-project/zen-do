from pytest import mark, raises

from zen_do.examples import example_metadata
from zen_do.zenodo_metadata import ZenodoMetadata, ZenodoRelatedIdentifier


def test_raises_error_if_zenodo_json_has_no_urn_id():
    metadata = example_metadata()
    del metadata.related_identifiers[0]

    with raises(ValueError):
        ZenodoMetadata.model_validate(metadata)


def test_raises_error_if_zenodo_json_has_multiple_urn_ids():
    metadata = example_metadata()
    metadata.related_identifiers.append(
        ZenodoRelatedIdentifier(
            identifier="urn:zenodo:my-org:project:poster",
            relation="isIdenticalTo",
            resource_type="other",
            scheme="urn",
        )
    )

    with raises(ValueError):
        ZenodoMetadata.model_validate(metadata)


@mark.parametrize(
    "urn",
    [
        "",
        "not a URN",
        "urn",
        "urn:",
        "urn:unknown",
        "urn:zenodo",
        "urn:zenodo:",
        "urn:zenodo:a:",
        "urn:zenodo:a/b",
    ],
)
def test_flags_incorrect_urn(urn):
    with raises(ValueError):
        example_metadata(urn=urn)
