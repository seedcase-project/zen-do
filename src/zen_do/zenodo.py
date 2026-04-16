import re
from pathlib import Path
from typing import Literal, Optional, Self, Union

import requests
from pydantic import BaseModel, ConfigDict, model_validator

from zen_do.internals import _filter, _map


class ZenodoModel(BaseModel):
    """Model configuring all Zenodo models."""

    model_config = ConfigDict(extra="allow", frozen=True)


class ZenodoCreator(ZenodoModel):
    """Model representing the creator of a Zenodo deposit.

    Attributes:
        name: The name of the creator.
        affiliation: The affiliation of the creator.
        orcid: The ORCID of the creator.
    """

    name: str
    affiliation: str
    orcid: str


class ZenodoRelatedIdentifier(ZenodoModel):
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


class ZenodoMetadata(ZenodoModel):
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


class ZenodoLinks(ZenodoModel):
    """Model representing the group of links in Zenodo metadata.

    Attributes:
        bucket: The file upload link for the deposit.
        latest_draft: Link to the latest draft or the deposit.
    """

    # Published deposits cannot receive new file uploads
    bucket: Optional[str] = None
    latest_draft: str


class ZenodoFile(ZenodoModel):
    """Model representing a file on a Zenodo deposit."""


type ZenodoDepositState = Literal["done", "inprogress", "error", "unsubmitted"]


class ZenodoDeposit(ZenodoModel):
    """Model representing a Zenodo deposit.

    Attributes:
        id: The ID of the deposit.
        metadata: The metadata the deposit.
        links: Links to deposit assets and API endpoints.
        state: The state of the deposit.
        submitted: Whether the deposit has been published.
    """

    id: int
    metadata: ZenodoMetadata
    links: ZenodoLinks
    state: ZenodoDepositState
    submitted: bool

    @property
    def editable(self) -> bool:
        """Whether the deposit can be edited."""
        return self.state in ["inprogress", "unsubmitted"]


def zenodo_get_deposit(token: str) -> Optional[ZenodoDeposit]:
    """Gets the Zenodo deposit for the repository if it exists.

    Gets the repository URL from the `.zenodo.json` file. If one
    doesn't exist, this function will not work.

    Args:
        token: Zenodo API access token.

    Returns:
        The Zenodo deposit for the repo if it exists, None otherwise.
    """
    urn = _get_urn()

    response = requests.get(
        "https://zenodo.org/api/deposit/depositions",
        headers={"Authorization": f"Bearer {token}"},
        timeout=10,
    )
    response.raise_for_status()
    deposits: list[ZenodoDeposit] = _map(response.json(), ZenodoDeposit.model_validate)
    matching_deposits = _filter(
        deposits,
        lambda deposit: bool(
            _filter(
                deposit.metadata.related_identifiers,
                lambda id: _is_urn(id) and id.identifier == urn,
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


def zenodo_create_deposit(token: str) -> None:
    """Create a new Zenodo concept deposit and publish the first version.

    Args:
        token: Zenodo API access token.
    """
    ...


def zenodo_update_deposit(token: str, id: int) -> None:
    """Publish a new version of an existing Zenodo concept deposit.

    Args:
        token: Zenodo API access token.
        id: Zenodo deposit ID.
    """
    ...


def _load_zenodo_json() -> ZenodoMetadata:
    return ZenodoMetadata.model_validate_json(Path(".zenodo.json").read_text())


def _is_urn(id: ZenodoRelatedIdentifier) -> bool:
    return id.relation == "isIdenticalTo" and id.scheme == "urn"


def _get_urn() -> str:
    metadata = _load_zenodo_json()
    ids = _filter(metadata.related_identifiers, _is_urn)
    if len(ids) != 1:
        raise ValueError(
            "Expected exactly one `isIdenticalTo` URN in `.zenodo.json` under "
            f"`related_identifiers`, but found {len(ids)}. Ensure there is a single "
            "unique URN, as it is used to identify the corresponding deposit on Zenodo."
        )

    return ids[0].identifier


class ZenodoClient:
    """Class for interacting with the Zenodo API."""

    def __init__(self, sandbox: bool, token: str, timeout: int = 30):
        """Initialises the client.

        Args:
            sandbox: Whether to use the sandbox API or the real one.
            token: Zenodo access token.
            timeout: Request timeout in seconds.
        """
        self.headers = {"Authorization": f"Bearer {token}"}
        self.timeout = timeout

        host = "sandbox.zenodo" if sandbox else "zenodo"
        self.depositions = f"https://{host}.org/api/deposit/depositions"

    def _resolve[ResponseType: ZenodoModel](
        self,
        response: requests.Response,
        response_type: type[ResponseType],
    ) -> ResponseType:
        """Maps the API response to the given model."""
        # TODO: include response.text in error because that is where Zenodo
        # gives reasons
        response.raise_for_status()
        return response_type.model_construct(**response.json())

    def _resolve_list[ResponseType: ZenodoModel](
        self,
        response: requests.Response,
        response_type: type[ResponseType],
    ) -> list[ResponseType]:
        """Maps the API response to a list of the given model."""
        # TODO: include response.text in error because that is where Zenodo
        # gives reasons
        response.raise_for_status()
        return _map(response.json(), lambda item: response_type.model_construct(**item))

    def get_deposits(self) -> list[ZenodoDeposit]:
        """Gets all deposits.

        Returns:
            The list of all deposits.
        """
        response = requests.get(
            self.depositions, headers=self.headers, timeout=self.timeout
        )
        return self._resolve_list(response, ZenodoDeposit)

    def get_deposit(self, deposit_id: Union[int, str]) -> ZenodoDeposit:
        """Gets the deposit with the given ID.

        Args:
            deposit_id: The ID of the deposit.

        Returns:
            The deposit.

        Raises:
            requests.exceptions.HTTPError: If there is no deposit with the given ID.
        """
        response = requests.get(
            f"{self.depositions}/{deposit_id}",
            headers=self.headers,
            timeout=self.timeout,
        )
        return self._resolve(response, ZenodoDeposit)

    def create(self, metadata: ZenodoMetadata) -> ZenodoDeposit:
        """Creates a new deposition in editable state.

        Args:
            metadata: The metadata of the new deposition.

        Returns:
            The newly created deposition.
        """
        response = requests.post(
            self.depositions,
            headers=self.headers,
            json={"metadata": metadata.model_dump()},
            timeout=self.timeout,
        )
        return self._resolve(response, ZenodoDeposit)

    def make_editable(self, deposition: ZenodoDeposit) -> ZenodoDeposit:
        """Makes the deposition editable.

        Args:
            deposition: The deposition.

        Returns:
            The deposition in editable state.
        """
        if deposition.editable:
            return deposition

        response = requests.post(
            f"{self.depositions}/{deposition.id}/actions/edit",
            headers=self.headers,
            timeout=self.timeout,
        )
        return self._resolve(response, ZenodoDeposit)

    def discard(self, deposition: ZenodoDeposit) -> None:
        """Puts the deposition in a non-editable state by discarding all changes.

        If the deposition's state is `unsubmitted`, the deposition is deleted.
        If the deposition's state is `inprogress`, the deposition is restored to
        the state when it was last published.

        Args:
            deposition: The deposition.
        """
        if not deposition.editable:
            return None

        response = requests.post(
            f"{self.depositions}/{deposition.id}/actions/discard",
            headers=self.headers,
            timeout=self.timeout,
        )
        response.raise_for_status()

    def update_metadata(
        self, deposition: ZenodoDeposit, metadata: ZenodoMetadata
    ) -> ZenodoDeposit:
        """Updates the metadata of a deposition.

        Args:
            deposition: The deposition.
            metadata: The new metadata.

        Returns:
            The updated deposition.
        """
        deposition = self.make_editable(deposition)
        response = requests.put(
            f"{self.depositions}/{deposition.id}",
            headers=self.headers,
            json={"metadata": metadata.model_dump()},
            timeout=self.timeout,
        )
        return self._resolve(response, ZenodoDeposit)

    def upload_file(self, deposition: ZenodoDeposit, file_path: Path) -> ZenodoFile:
        """Uploads a file to a deposition. The deposition must be unpublished.

        Args:
            deposition: The deposition.
            file_path: The path to the file.

        Returns:
            The updated deposition.
        """
        if deposition.submitted:
            raise ValueError(
                f"Cannot upload new file to deposition {deposition.id} because the "
                "deposition has already been published. You must first create a new "
                "version of the deposition and upload the files there."
            )

        if not deposition.links.bucket:
            raise ValueError(
                f"Cannot upload new file to deposition {deposition.id} because the "
                "deposition does not have a file-upload (bucket) link. "
            )

        with file_path.open("rb") as file_stream:
            response = requests.put(
                f"{deposition.links.bucket}/{file_path.name}",
                data=file_stream,
                headers=self.headers,
                timeout=self.timeout,
            )
        return self._resolve(response, ZenodoFile)

    def publish(self, deposit: ZenodoDeposit) -> ZenodoDeposit:
        """Publishes a deposit.

        Args:
            deposit: The deposit.

        Returns:
            The published deposit.
        """
        if deposit.submitted and deposit.state == "done":
            return deposit

        response = requests.post(
            f"{self.depositions}/{deposit.id}/actions/publish",
            headers=self.headers,
            timeout=self.timeout,
        )
        return self._resolve(response, ZenodoDeposit)
