from pathlib import Path
from typing import Optional

import seedcase_soil as ss

from zen_do.zenodo_client import ZenodoClient, ZenodoResponse, _get_zenodo_field
from zen_do.zenodo_metadata import ZenodoMetadata, ZenodoRelatedIdentifier


def zenodo_get_deposit(client: ZenodoClient) -> Optional[ZenodoResponse]:
    """Gets the Zenodo deposit for the repository if it exists.

    Gets the repository URL from the `.zenodo.json` file. If one
    doesn't exist, this function will not work.

    Args:
        client: The Zenodo client to use for the request.

    Returns:
        The Zenodo deposit for the repo if it exists, None otherwise.
    """
    urn = _get_urn()

    deposits = client.get_deposits()
    matching_deposits = ss.keep(
        deposits,
        lambda deposit: bool(
            ss.keep(
                _get_zenodo_field(deposit, "metadata").get("related_identifiers", []),
                lambda id: _urn_matches(id, urn),
            )
        ),
    )

    if len(matching_deposits) > 1:
        raise ValueError(
            "There are multiple deposits on Zenodo with the URN "
            f"{urn!r} as a `related identifier`. We only allow one."
        )
    if not matching_deposits:
        return None
    return matching_deposits[0]


def _urn_matches(id_response: ZenodoResponse, target_urn: str) -> bool:
    id = ZenodoRelatedIdentifier.model_construct(**id_response)
    return _is_urn(id) and id.identifier == target_urn


def _is_urn(id: ZenodoRelatedIdentifier) -> bool:
    return id.relation == "isIdenticalTo" and id.scheme == "urn"


def _get_urn() -> str:
    metadata = _load_zenodo_json()
    ids = ss.keep(metadata.related_identifiers, _is_urn)
    if len(ids) != 1:
        raise ValueError(
            "Expected exactly one `isIdenticalTo` URN in `.zenodo.json` under "
            f"`related_identifiers`, but found {len(ids)}. Ensure there is a single "
            "unique URN, as it is used to identify the corresponding deposit on Zenodo."
        )

    return ids[0].identifier


def _load_zenodo_json() -> ZenodoMetadata:
    return ZenodoMetadata.model_validate_json(Path(".zenodo.json").read_text())
