import json
from http import HTTPStatus
from pathlib import Path

from pytest import MonkeyPatch, fixture, mark, raises
from requests import HTTPError
from requests_mock import ANY

from zen_do.examples import example_deposit, example_metadata
from zen_do.zenodo_client import ZenodoClient
from zen_do.zenodo_get_deposit import zenodo_get_deposit
from zen_do.zenodo_metadata import ZenodoRelatedIdentifier


@fixture
def _zenodo_json(monkeypatch: MonkeyPatch, tmp_path: Path) -> None:
    monkeypatch.chdir(tmp_path)
    (tmp_path / ".zenodo.json").write_text(example_metadata().model_dump_json())


sandbox_client = ZenodoClient("token", sandbox=True)


def test_returns_deposit_if_matching_deposit_has_exactly_one_matching_identifier(
    requests_mock, _zenodo_json
):
    requests_mock.get(
        url=ANY,
        json=[
            example_deposit(id=12),
            example_deposit(urn="urn:zenodo:my-org:project:poster"),
        ],
    )

    deposit = zenodo_get_deposit(sandbox_client)

    assert deposit
    assert deposit["id"] == 12


def test_returns_deposit_if_matching_deposit_has_at_least_one_matching_identifier(
    requests_mock, _zenodo_json
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
        ]
    )
    requests_mock.get(url=ANY, json=[fetched_deposit])

    deposit = zenodo_get_deposit(sandbox_client)

    assert deposit
    assert deposit["id"] == 12


def test_raises_error_if_multiple_matching_deposits(requests_mock, _zenodo_json):
    requests_mock.get(
        url=ANY,
        json=[example_deposit(id=12), example_deposit(id=13)],
    )

    with raises(ValueError):
        zenodo_get_deposit(sandbox_client)


def test_returns_none_if_no_deposits_on_zenodo(requests_mock, _zenodo_json):
    requests_mock.get(url=ANY, json=[])

    deposit = zenodo_get_deposit(sandbox_client)

    assert deposit is None


def test_returns_none_if_no_matching_deposits(requests_mock, _zenodo_json):
    requests_mock.get(
        url=ANY,
        json=[example_deposit(urn="urn:zenodo:my-org:project:poster")],
    )

    deposit = zenodo_get_deposit(sandbox_client)

    assert deposit is None


def test_raises_error_if_request_unsuccessful(requests_mock, _zenodo_json):
    requests_mock.get(url=ANY, status_code=HTTPStatus.BAD_REQUEST)

    with raises(HTTPError):
        zenodo_get_deposit(sandbox_client)


def test_raises_error_if_zenodo_json_has_no_repo_url(monkeypatch, tmp_path):
    metadata = example_metadata()
    monkeypatch.chdir(tmp_path)
    del metadata.related_identifiers[0]
    (tmp_path / ".zenodo.json").write_text(metadata.model_dump_json())

    with raises(ValueError):
        zenodo_get_deposit(sandbox_client)


def test_raises_error_if_zenodo_json_has_multiple_repo_urls(monkeypatch, tmp_path):
    metadata = example_metadata()
    monkeypatch.chdir(tmp_path)
    metadata.related_identifiers.append(
        ZenodoRelatedIdentifier(
            identifier="urn:zenodo:my-org:project:poster",
            relation="isIdenticalTo",
            resource_type="other",
            scheme="urn",
        )
    )
    (tmp_path / ".zenodo.json").write_text(metadata.model_dump_json())

    with raises(ValueError):
        zenodo_get_deposit(sandbox_client)


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
def test_flags_incorrect_urn(monkeypatch, tmp_path, urn):
    metadata_json = example_metadata().model_dump()
    metadata_json["related_identifiers"][0]["identifier"] = urn
    monkeypatch.chdir(tmp_path)
    (tmp_path / ".zenodo.json").write_text(json.dumps(metadata_json))

    with raises(ValueError):
        zenodo_get_deposit(sandbox_client)
