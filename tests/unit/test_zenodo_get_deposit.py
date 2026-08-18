from pytest import raises

from zen_do.examples import example_deposit, example_metadata
from zen_do.zenodo_get_deposit import zenodo_get_deposit
from zen_do.zenodo_metadata import ZenodoRelatedIdentifier


def test_returns_deposit_if_matching_deposit_has_exactly_one_matching_identifier():
    deposits = [
        example_deposit(id=12),
        example_deposit(urn="urn:zenodo:my-org:project:poster"),
    ]

    deposit = zenodo_get_deposit(deposits, example_metadata())

    assert deposit
    assert deposit["id"] == 12


def test_returns_deposit_if_matching_deposit_has_at_least_one_matching_identifier():
    fetched_deposit = example_deposit(id=12)
    fetched_deposit["metadata"]["related_identifiers"].extend(
        [
            # Duplicate identifier
            fetched_deposit["metadata"]["related_identifiers"][0],
            # Different identifier
            ZenodoRelatedIdentifier(
                identifier="urn:zenodo:my-org:project:poster",
                relation="isIdenticalTo",
                resource_type="other",
                scheme="urn",
            ).model_dump(),
            # Malformed identifier added in the UI
            {
                "identifier": "not our URN format",
                "relation": "isIdenticalTo",
                "scheme": "urn",
            },
        ]
    )

    deposit = zenodo_get_deposit([fetched_deposit], example_metadata())

    assert deposit
    assert deposit["id"] == 12


def test_raises_error_if_multiple_matching_deposits():
    deposits = [example_deposit(id=12), example_deposit(id=13)]

    with raises(ValueError):
        zenodo_get_deposit(deposits, example_metadata())


def test_returns_none_if_no_deposits_on_zenodo():
    deposit = zenodo_get_deposit([], example_metadata())

    assert deposit is None


def test_returns_none_if_no_matching_deposits():
    deposits = [example_deposit(urn="urn:zenodo:my-org:project:poster")]

    deposit = zenodo_get_deposit(deposits, example_metadata())

    assert deposit is None
