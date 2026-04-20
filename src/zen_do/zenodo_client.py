from pathlib import Path
from typing import Any, Literal, Union

import requests

from zen_do.zenodo_metadata import ZenodoMetadata

type ZenodoResponse = dict[str, Any]


type ZenodoDepositState = Literal["done", "inprogress", "error", "unsubmitted"]


def _get_deposit_field(deposit: ZenodoResponse, field: str) -> Any:
    if field not in deposit:
        raise ValueError(f"Missing '{field}' field in object {deposit!r}.")
    return deposit[field]


def _get_deposit_id(deposit: ZenodoResponse) -> int:
    return int(_get_deposit_field(deposit, "id"))


def _is_deposit_editable(deposit: ZenodoResponse) -> bool:
    return _get_deposit_field(deposit, "state") in ["inprogress", "unsubmitted"]


class ZenodoClient:
    """Class for interacting with the Zenodo API."""

    def __init__(self, token: str, sandbox: bool = False, timeout: int = 30):
        """Initialises the client.

        Args:
            token: Zenodo access token.
            sandbox: Whether to use the sandbox API or the real one.
            timeout: Request timeout in seconds.
        """
        self.headers = {"Authorization": f"Bearer {token}"}
        self.timeout = timeout

        host = "sandbox.zenodo" if sandbox else "zenodo"
        self.deposits = f"https://{host}.org/api/deposit/depositions"

    def _resolve(
        self,
        response: requests.Response,
    ) -> ZenodoResponse:
        """Maps the API response to a dictionary."""
        # TODO: include response.text in error because that is where Zenodo
        # gives reasons
        response.raise_for_status()
        response_json = response.json()
        if not isinstance(response_json, dict):
            raise TypeError(
                f"Expected response to be a JSON object but got {response_json!r}"
            )
        return response_json

    def _resolve_list(
        self,
        response: requests.Response,
    ) -> list[ZenodoResponse]:
        """Maps the API response to a list."""
        # TODO: include response.text in error because that is where Zenodo
        # gives reasons
        response.raise_for_status()
        response_json = response.json()
        if not isinstance(response_json, list):
            raise TypeError(
                f"Expected response to be a JSON array but got {response_json!r}"
            )
        return response_json

    def get_deposits(self) -> list[ZenodoResponse]:
        """Gets all deposits.

        Returns:
            The list of all deposits.
        """
        response = requests.get(
            self.deposits, headers=self.headers, timeout=self.timeout
        )
        return self._resolve_list(response)

    def get_deposit(self, deposit_id: Union[int, str]) -> ZenodoResponse:
        """Gets the deposit with the given ID.

        Args:
            deposit_id: The ID of the deposit.

        Returns:
            The deposit.

        Raises:
            requests.exceptions.HTTPError: If there is no deposit with the given ID.
        """
        response = requests.get(
            f"{self.deposits}/{deposit_id}",
            headers=self.headers,
            timeout=self.timeout,
        )
        return self._resolve(response)

    def create(self, metadata: ZenodoMetadata) -> ZenodoResponse:
        """Creates a new deposit in editable state.

        Args:
            metadata: The metadata of the new deposit.

        Returns:
            The newly created deposit.
        """
        response = requests.post(
            self.deposits,
            headers=self.headers,
            json={"metadata": metadata.model_dump()},
            timeout=self.timeout,
        )
        return self._resolve(response)

    def make_editable(self, deposit: ZenodoResponse) -> ZenodoResponse:
        """Makes the deposit editable.

        Args:
            deposit: The deposit.

        Returns:
            The deposit in editable state.
        """
        if _is_deposit_editable(deposit):
            return deposit

        response = requests.post(
            f"{self.deposits}/{_get_deposit_id(deposit)}/actions/edit",
            headers=self.headers,
            timeout=self.timeout,
        )
        return self._resolve(response)

    def discard(self, deposit: ZenodoResponse) -> None:
        """Puts the deposit in a non-editable state by discarding all changes.

        If the deposit's state is `unsubmitted`, the deposit is deleted.
        If the deposit's state is `inprogress`, the deposit is restored to
        the state when it was last published.

        Args:
            deposit: The deposit.
        """
        if not _is_deposit_editable(deposit):
            return None

        response = requests.post(
            f"{self.deposits}/{_get_deposit_id(deposit)}/actions/discard",
            headers=self.headers,
            timeout=self.timeout,
        )
        response.raise_for_status()

    def update_metadata(
        self, deposit: ZenodoResponse, metadata: ZenodoMetadata
    ) -> ZenodoResponse:
        """Updates the metadata of a deposit.

        Args:
            deposit: The deposit.
            metadata: The new metadata.

        Returns:
            The updated deposit.
        """
        deposit = self.make_editable(deposit)
        response = requests.put(
            f"{self.deposits}/{_get_deposit_id(deposit)}",
            headers=self.headers,
            json={"metadata": metadata.model_dump()},
            timeout=self.timeout,
        )
        return self._resolve(response)

    def new_version(self, deposit: ZenodoResponse) -> ZenodoResponse:
        """Creates a new, unpublished version of a published deposit.

        Args:
            deposit: The deposit.

        Returns:
            The new version of the deposit.
        """
        id = _get_deposit_id(deposit)
        if not _get_deposit_field(deposit, "submitted"):
            raise ValueError(
                f"Cannot create new version for deposit {id} because it "
                "has not yet been published."
            )

        # Discard any changes on the old version to avoid the situation where we
        # create a new version but leave unpublished changes on the old one.
        self.discard(deposit)
        response = requests.post(
            f"{self.deposits}/{id}/actions/newversion",
            headers=self.headers,
            timeout=self.timeout,
        )
        return self._resolve(response)

    def upload_file(self, deposit: ZenodoResponse, file_path: Path) -> ZenodoResponse:
        """Uploads a file to a deposit. The deposit must be unpublished.

        Args:
            deposit: The deposit.
            file_path: The path to the file.

        Returns:
            Information about the uploaded file.
        """
        id = _get_deposit_id(deposit)
        if _get_deposit_field(deposit, "submitted"):
            raise ValueError(
                f"Cannot upload new file to deposit {id} because the "
                "deposit has already been published. You must first create a new "
                "version of the deposit and upload the files there."
            )

        bucket = _get_deposit_field(deposit, "links").get("bucket")
        if not bucket:
            raise ValueError(
                f"Cannot upload new file to deposit {id} because the "
                "deposit does not have a file-upload (bucket) link. "
            )

        with file_path.open("rb") as file_stream:
            response = requests.put(
                f"{bucket}/{file_path.name}",
                data=file_stream,
                headers=self.headers,
                timeout=self.timeout,
            )
        return self._resolve(response)

    def publish(self, deposit: ZenodoResponse) -> ZenodoResponse:
        """Publishes a deposit.

        Args:
            deposit: The deposit.

        Returns:
            The published deposit.
        """
        if (
            _get_deposit_field(deposit, "submitted")
            and _get_deposit_field(deposit, "state") == "done"
        ):
            return deposit

        response = requests.post(
            f"{self.deposits}/{_get_deposit_id(deposit)}/actions/publish",
            headers=self.headers,
            timeout=self.timeout,
        )
        return self._resolve(response)
