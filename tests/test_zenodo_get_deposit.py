from pathlib import Path

import pydantic
import toml
from pytest import MonkeyPatch, fixture, raises

from zen_do.examples import example_deposit, example_metadata
from zen_do.zenodo_get_deposit import zenodo_get_deposit
from zen_do.zenodo_metadata import ZenodoRelatedIdentifier


@fixture
def _zenodo_toml(monkeypatch: MonkeyPatch, tmp_path: Path) -> None:
    monkeypatch.chdir(tmp_path)
    (tmp_path / ".zenodo.toml").write_text(toml.dumps(example_metadata().model_dump()))


def test_returns_deposit_if_matching_deposit_has_exactly_one_matching_identifier(
    _zenodo_toml,
):
    deposits = [
        example_deposit(id=12),
        example_deposit(urn="urn:zenodo:my-org:project:poster"),
    ]

    deposit = zenodo_get_deposit(deposits)

    assert deposit
    assert deposit["id"] == 12


def test_can_use_non_default_file_location(tmp_path):
    (tmp_path / "book").mkdir()
    metadata_path = tmp_path / "book" / ".book.zenodo.toml"
    metadata_path.write_text(toml.dumps(example_metadata().model_dump()))

    deposit = zenodo_get_deposit([example_deposit(id=12)], metadata_path)

    assert deposit
    assert deposit["id"] == 12


def test_returns_deposit_if_matching_deposit_has_at_least_one_matching_identifier(
    _zenodo_toml,
):
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

    deposit = zenodo_get_deposit([fetched_deposit])

    assert deposit
    assert deposit["id"] == 12


def test_raises_error_if_multiple_matching_deposits(_zenodo_toml):
    deposits = [example_deposit(id=12), example_deposit(id=13)]

    with raises(ValueError):
        zenodo_get_deposit(deposits)


def test_raises_error_if_metadata_file_empty(tmp_path):
    metadata_path = tmp_path / ".zenodo.toml"
    metadata_path.touch()

    with raises(pydantic.ValidationError):
        zenodo_get_deposit([example_deposit()], metadata_path)


def test_returns_none_if_no_deposits_on_zenodo(_zenodo_toml):
    deposit = zenodo_get_deposit([])

    assert deposit is None


def test_returns_none_if_no_matching_deposits(_zenodo_toml):
    deposits = [example_deposit(urn="urn:zenodo:my-org:project:poster")]

    deposit = zenodo_get_deposit(deposits)

    assert deposit is None
