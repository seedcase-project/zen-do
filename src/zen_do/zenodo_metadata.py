import re
from typing import Optional, Self

import seedcase_soil as so
from pydantic import BaseModel, ConfigDict, model_validator


class KebabModel(BaseModel, frozen=True):
    """Allow creating Pydantic model from kebab-case data."""

    model_config = ConfigDict(
        alias_generator=lambda string: string.replace("_", "-"),
        populate_by_name=True,
    )


class ZenodoCreator(KebabModel, frozen=True):
    """Model representing the creator of a Zenodo deposit.

    Attributes:
        name: The name of the creator.
        affiliation: The affiliation of the creator.
        orcid: The ORCID of the creator.
    """

    name: str
    affiliation: str
    orcid: str


class ZenodoRelatedIdentifier(KebabModel, frozen=True):
    """Model representing an identifier related to a Zenodo deposit.

    Attributes:
        identifier: The value of the identifier.
        relation: The relationship between the deposit and the other piece of work
            identified by the identifier.
        resource_type: The type of the work identified by the identifier.
        scheme: The scheme followed by the identifier.
    """

    identifier: str
    relation: str
    resource_type: str
    scheme: Optional[str] = None

    @model_validator(mode="after")
    def _check_urn(self) -> Self:

        if self.scheme == "urn" and not re.fullmatch(
            r"urn:zenodo(:[^/:]+)+", self.identifier
        ):
            raise ValueError(
                f"The URN {self.identifier!r} does not have the expected format. URNs "
                "must be in the format 'urn:zenodo:<unique-id>(:<optional-sub-id>)'. "
                "We recommend 'urn:zenodo:<github-username>:<repo-name>:<output-type>'."
            )
        return self


class ZenodoMetadata(KebabModel, frozen=True):
    """Model representing Zenodo metadata.

    Attributes:
        title: The title of the deposit.
        upload_type: The type of the deposit.
        creators: The creators of the deposit.
        related_identifiers: Identifiers related to the deposit.
    """

    title: str
    upload_type: str
    creators: list[ZenodoCreator]
    related_identifiers: list[ZenodoRelatedIdentifier] = []

    @property
    def urn(self) -> str:
        """The URN related identifier of the deposit."""
        urns = so.keep(self.related_identifiers, _is_urn)
        return urns[0].identifier

    @model_validator(mode="after")
    def _check_unique_urn(self) -> Self:
        urns = so.keep(self.related_identifiers, _is_urn)
        if len(urns) != 1:
            raise ValueError(
                "Expected exactly one `isIdenticalTo` URN in the Zenodo metadata file "
                f"under `related_identifiers`, but found {len(urns)}. Ensure there is "
                "a single unique URN, as it is used to identify the corresponding "
                "deposit on Zenodo."
            )
        return self


def _is_urn(id: ZenodoRelatedIdentifier) -> bool:
    return id.relation == "isIdenticalTo" and id.scheme == "urn"
