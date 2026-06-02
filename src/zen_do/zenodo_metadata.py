import re
from typing import Any, Optional, Self

from pydantic import BaseModel, ConfigDict, model_validator


class KebabModel(BaseModel, frozen=True):
    """Allow creating Pydantic model from kebab-case data."""

    model_config = ConfigDict(
        alias_generator=lambda string: string.replace("_", "-"),
        populate_by_name=True,
    )

    def model_dump_toml(self) -> dict[str, Any]:
        """Dump the model to a dict with kebab-case keys for TOML."""
        return self.model_dump(by_alias=True)


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
