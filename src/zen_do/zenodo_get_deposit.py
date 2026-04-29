import tomllib
from pathlib import Path
from typing import Optional

import seedcase_soil as so

from zen_do.zenodo_client import ZenodoResponse, _get_zenodo_field
from zen_do.zenodo_metadata import ZenodoMetadata, ZenodoRelatedIdentifier, _is_urn


def zenodo_get_deposit(deposits: list[ZenodoResponse]) -> Optional[ZenodoResponse]:
    """Gets the Zenodo deposit for the repository if it exists.

    Gets the URN identifier from the `.zenodo.toml` file. If one
    doesn't exist, this function will not work.

    Args:
        deposits: All the deposits on Zenodo associated with an access token.

    Returns:
        The Zenodo deposit for the repo if it exists, None otherwise.
    """
    urn = _load_zenodo_toml().urn

    matching_deposits = so.keep(
        deposits,
        lambda deposit: bool(
            so.keep(
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


def _load_zenodo_toml() -> ZenodoMetadata:
    with open(Path(".zenodo.toml"), mode="rb") as file:
        toml_file = tomllib.load(file)

    return ZenodoMetadata.model_validate(toml_file)
