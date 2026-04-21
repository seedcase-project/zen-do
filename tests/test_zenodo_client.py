from pathlib import Path
from typing import Any

import pytest
import requests
from pytest import mark, raises

from zen_do.examples import example_deposit, example_metadata
from zen_do.zenodo_client import ZenodoClient

sandbox_client = ZenodoClient(sandbox=True, token="token")


def test_creates_client_for_sandbox_api():
    assert sandbox_client.deposits.startswith("https://sandbox.zenodo")
    assert "token" in sandbox_client.headers["Authorization"]


def test_creates_client_for_real_api():
    client = ZenodoClient(sandbox=False, token="token")

    assert client.deposits.startswith("https://zenodo")
    assert "token" in client.headers["Authorization"]


# Endpoints


def assert_headers_correct(mock: Any) -> None:
    assert mock.called
    request = mock.last_request
    assert request.headers["Authorization"] == sandbox_client.headers["Authorization"]


@pytest.fixture
def mock_get_deposits(requests_mock):
    def _mock(json=[], status_code=200):
        return requests_mock.get(
            sandbox_client.deposits, json=json, status_code=status_code
        )

    return _mock


@pytest.fixture
def mock_get_deposit(requests_mock):
    deposit = example_deposit()

    def _mock(json=deposit, id=deposit["id"], status_code=200):
        return requests_mock.get(
            f"{sandbox_client.deposits}/{id}",
            json=json,
            status_code=status_code,
        )

    return _mock


@pytest.fixture
def mock_create(requests_mock):
    def _mock(json=example_deposit(), status_code=201):
        return requests_mock.post(
            sandbox_client.deposits, json=json, status_code=status_code
        )

    return _mock


@pytest.fixture
def mock_make_editable(requests_mock):
    deposit = example_deposit()

    def _mock(json=deposit, id=deposit["id"], status_code=201):
        return requests_mock.post(
            f"{sandbox_client.deposits}/{id}/actions/edit",
            json=json,
            status_code=status_code,
        )

    return _mock


@pytest.fixture
def mock_discard(requests_mock):
    def _mock(id=example_deposit()["id"], status_code=204):
        return requests_mock.post(
            f"{sandbox_client.deposits}/{id}/actions/discard",
            status_code=status_code,
        )

    return _mock


@pytest.fixture
def mock_update_metadata(requests_mock):
    deposit = example_deposit()

    def _mock(json=deposit, id=deposit["id"], status_code=200):
        return requests_mock.put(
            f"{sandbox_client.deposits}/{id}",
            json=json,
            status_code=status_code,
        )

    return _mock


@pytest.fixture
def mock_new_version(requests_mock):
    deposition = example_deposit()

    def _mock(json=deposition, id=deposition["id"], status_code=201):
        return requests_mock.post(
            f"{sandbox_client.deposits}/{id}/actions/newversion",
            json=json,
            status_code=status_code,
        )

    return _mock


@pytest.fixture
def mock_upload_file(requests_mock):
    def _mock(
        url=None,
        json={"version_id": "abcd"},
        file_path=Path("data.txt"),
        status_code=200,
    ):
        if url is None:
            url = f"{example_deposit()['links']['bucket']}/{file_path.name}"
        return requests_mock.put(
            url,
            json=json,
            status_code=status_code,
        )

    return _mock


@pytest.fixture
def mock_publish(requests_mock):
    deposit = example_deposit()

    def _mock(json=deposit, id=deposit["id"], status_code=202):
        return requests_mock.post(
            f"{sandbox_client.deposits}/{id}/actions/publish",
            json=json,
            status_code=status_code,
        )

    return _mock


def test_flags_missing_field_in_response():
    # Test _get_zenodo_field
    with raises(KeyError):
        sandbox_client.publish({})

    # Test _get_deposit_id
    with raises(KeyError):
        sandbox_client.new_version({})


# _resolve and _resolve_list


def test_resolve_does_not_check_response_properties(mock_get_deposit):
    mock_get_deposit({"unexpected": "response"})

    assert sandbox_client.get_deposit(123)


def test_resolve_list_does_not_check_response_properties(mock_get_deposits):
    mock_get_deposits([{"unexpected": "response"}])

    assert sandbox_client.get_deposits()


def test_resolve_flags_unexpected_type(mock_get_deposit):
    mock_get_deposit([{"not": "a dict"}])

    with raises(TypeError):
        sandbox_client.get_deposit(123)


def test_resolve_list_flags_unexpected_type(mock_get_deposits):
    mock_get_deposits({"not": "a list"})

    with raises(TypeError):
        sandbox_client.get_deposits()


# get_deposits


def test_get_deposits_success(mock_get_deposits):
    mock_get_deposits([])
    result = sandbox_client.get_deposits()
    assert result == []

    mock = mock_get_deposits([example_deposit(1), example_deposit(2)])
    result = sandbox_client.get_deposits()
    assert_headers_correct(mock)
    assert len(result) == 2
    assert result[0]["id"] == 1
    assert result[1]["id"] == 2


def test_get_deposits_failure(mock_get_deposits):
    mock_get_deposits(status_code=500)
    with raises(requests.HTTPError):
        sandbox_client.get_deposits()


# get_deposit


def test_get_deposit_success(mock_get_deposit):
    mock = mock_get_deposit()

    result = sandbox_client.get_deposit(123)

    assert_headers_correct(mock)
    assert result["id"] == 123


def test_get_deposit_failure(mock_get_deposit):
    mock_get_deposit(status_code=500)
    with raises(requests.HTTPError):
        sandbox_client.get_deposit(123)


# create


def test_create_success(mock_create):
    metadata = example_metadata()
    mock = mock_create()

    result = sandbox_client.create(metadata)

    assert_headers_correct(mock)
    assert result["id"] == 123
    assert mock.last_request.json()["metadata"] == metadata.model_dump()


def test_create_failure(mock_create):
    metadata = example_metadata()
    mock_create(status_code=400)
    with raises(requests.HTTPError):
        sandbox_client.create(metadata)


# make_editable


@mark.parametrize("state", ["inprogress", "unsubmitted"])
def test_make_editable_success_when_editable(mock_make_editable, state):
    deposit = example_deposit(state=state)
    mock = mock_make_editable()

    result = sandbox_client.make_editable(deposit)

    assert not mock.called
    assert result["id"] == deposit["id"]


def test_make_editable_success_when_not_editable(mock_make_editable):
    deposit = example_deposit(state="done")
    mock = mock_make_editable()

    result = sandbox_client.make_editable(deposit)

    assert_headers_correct(mock)
    assert result["id"] == deposit["id"]


def test_make_editable_failure(mock_make_editable):
    deposit = example_deposit()
    mock_make_editable(status_code=400)
    with raises(requests.HTTPError):
        sandbox_client.make_editable(deposit)


# discard


@mark.parametrize("state", ["inprogress", "unsubmitted"])
def test_discard_success_when_editable(mock_discard, state):
    mock = mock_discard()

    sandbox_client.discard(example_deposit(state=state))

    assert_headers_correct(mock)


def test_discard_success_when_not_editable(mock_discard):
    mock = mock_discard()

    sandbox_client.discard(example_deposit(state="done"))

    assert not mock.called


def test_discard_failure(mock_discard):
    mock_discard(status_code=400)
    with raises(requests.HTTPError):
        sandbox_client.discard(example_deposit(state="inprogress"))


# update_metadata


@mark.parametrize("state", ["inprogress", "unsubmitted", "done"])
def test_update_metadata_success(mock_update_metadata, mock_make_editable, state):
    deposit = example_deposit(state=state)
    new_metadata = example_metadata(title="New Title")
    mock_make_editable()
    mock = mock_update_metadata()

    result = sandbox_client.update_metadata(deposit, new_metadata)

    assert_headers_correct(mock)
    assert result["id"] == deposit["id"]
    assert mock.last_request.json() == {"metadata": new_metadata.model_dump()}


def test_update_metadata_failure(mock_update_metadata, mock_make_editable):
    deposit = example_deposit()
    mock_make_editable()

    mock_update_metadata(status_code=400)
    with raises(requests.HTTPError):
        sandbox_client.update_metadata(deposit, example_metadata())


# new_version


def test_new_version_success(mock_discard, mock_new_version):
    mock_discard()
    new_version_response = example_deposit(id=88)
    mock_new_version = mock_new_version(new_version_response)

    result = sandbox_client.new_version(example_deposit(submitted=True))

    assert_headers_correct(mock_new_version)
    assert result["id"] == new_version_response["id"]


def test_new_version_flags_non_published(mock_discard, mock_new_version):
    mock_discard()
    mock_new_version()
    with raises(ValueError):
        sandbox_client.new_version(example_deposit(submitted=False))


def test_new_version_api_failure(mock_discard, mock_new_version):
    mock_discard()

    mock_new_version(status_code=400)
    with raises(requests.HTTPError):
        sandbox_client.new_version(example_deposit())


# upload_file


def test_upload_file_success(mock_upload_file, tmp_path):
    file_path = tmp_path / "data.txt"
    file_path.write_text("This is my file.")
    deposit = example_deposit(submitted=False, state="unsubmitted")
    mock = mock_upload_file()

    result = sandbox_client.upload_file(deposit, file_path=file_path)

    assert_headers_correct(mock)
    assert mock.last_request.body.name == str(file_path)
    assert result


def test_upload_file_failure_api(mock_upload_file, tmp_path):
    file_path = tmp_path / "data.txt"
    file_path.write_text("This is my file.")
    deposit = example_deposit(submitted=False, state="unsubmitted")
    mock_upload_file(status_code=400)
    with raises(requests.HTTPError):
        sandbox_client.upload_file(deposit, file_path=file_path)


def test_upload_file_failure_file_not_found():
    with raises(FileNotFoundError):
        sandbox_client.upload_file(
            example_deposit(submitted=False, state="unsubmitted"),
            file_path=Path("data.txt"),
        )


def test_upload_file_failure_published():
    with raises(ValueError):
        sandbox_client.upload_file(
            example_deposit(submitted=True),
            file_path=Path("data.txt"),
        )


def test_upload_file_failure_no_bucket():
    with raises(ValueError):
        sandbox_client.upload_file(
            example_deposit(submitted=False, state="unsubmitted", bucket=None),
            file_path=Path("data.txt"),
        )


# publish


@mark.parametrize(
    "deposit",
    [
        example_deposit(submitted=False, state="unsubmitted"),
        example_deposit(submitted=True, state="inprogress"),
    ],
)
def test_publish_success_unpublished(mock_publish, deposit):
    mock = mock_publish()

    result = sandbox_client.publish(deposit)

    assert_headers_correct(mock)
    assert result["id"] == 123


def test_publish_success_published(mock_publish):
    mock = mock_publish()

    result = sandbox_client.publish(example_deposit(submitted=True, state="done"))

    assert not mock.called
    assert result["id"] == 123


def test_publish_failure(mock_publish):
    deposit = example_deposit(submitted=True, state="inprogress")
    mock_publish(status_code=500)
    with raises(requests.HTTPError):
        sandbox_client.publish(deposit)
