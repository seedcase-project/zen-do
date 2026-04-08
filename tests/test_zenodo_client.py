from pathlib import Path
from typing import Any

import pydantic
import pytest
import requests
from pytest import mark, raises

from zen_do.examples import example_metadata, example_record
from zen_do.zenodo import ZenodoClient

sandbox_client = ZenodoClient(sandbox=True, token="token")


def test_creates_client_for_sandbox_api():
    assert sandbox_client.depositions.startswith("https://sandbox.zenodo")
    assert "token" in sandbox_client.headers["Authorization"]


def test_creates_client_for_real_api():
    client = ZenodoClient(sandbox=False, token="token")

    assert client.depositions.startswith("https://zenodo")
    assert "token" in client.headers["Authorization"]


# Endpoints


def assert_headers_correct(mock: Any) -> None:
    assert mock.called
    request = mock.last_request
    assert request.headers["Authorization"] == sandbox_client.headers["Authorization"]


@pytest.fixture
def mock_get_records(requests_mock):
    def _mock(json=[], status_code=200):
        return requests_mock.get(
            sandbox_client.depositions, json=json, status_code=status_code
        )

    return _mock


@pytest.fixture
def mock_get_record(requests_mock):
    record = example_record()

    def _mock(json=record.model_dump(), id=record.id, status_code=200):
        return requests_mock.get(
            f"{sandbox_client.depositions}/{id}",
            json=json,
            status_code=status_code,
        )

    return _mock


@pytest.fixture
def mock_create(requests_mock):
    def _mock(json=example_record().model_dump(), status_code=201):
        return requests_mock.post(
            sandbox_client.depositions, json=json, status_code=status_code
        )

    return _mock


@pytest.fixture
def mock_make_editable(requests_mock):
    deposition = example_record()

    def _mock(json=deposition.model_dump(), id=deposition.id, status_code=201):
        return requests_mock.post(
            f"{sandbox_client.depositions}/{id}/actions/edit",
            json=json,
            status_code=status_code,
        )

    return _mock


@pytest.fixture
def mock_discard(requests_mock):
    def _mock(id=example_record().id, status_code=204):
        return requests_mock.post(
            f"{sandbox_client.depositions}/{id}/actions/discard",
            status_code=status_code,
        )

    return _mock


@pytest.fixture
def mock_new_version(requests_mock):
    deposition = example_record()

    def _mock(json=deposition.model_dump(), id=deposition.id, status_code=201):
        return requests_mock.post(
            f"{sandbox_client.depositions}/{id}/actions/newversion",
            json=json,
            status_code=status_code,
        )

    return _mock


@pytest.fixture
def mock_upload_file(requests_mock):
    def _mock(url=None, json={}, file_path=Path("data.txt"), status_code=200):
        if url is None:
            url = f"{example_record().links.bucket}/{file_path.name}"
        return requests_mock.put(
            url,
            json=json,
            status_code=status_code,
        )

    return _mock


@pytest.fixture
def mock_publish(requests_mock):
    record = example_record()

    def _mock(json=record.model_dump(), id=record.id, status_code=202):
        return requests_mock.post(
            f"{sandbox_client.depositions}/{id}/actions/publish",
            json=json,
            status_code=status_code,
        )

    return _mock


# get_records


def test_get_records_success(mock_get_records):
    mock_get_records([])
    result = sandbox_client.get_records()
    assert result == []

    mock = mock_get_records(
        [example_record(1).model_dump(), example_record(2).model_dump()]
    )
    result = sandbox_client.get_records()
    assert_headers_correct(mock)
    assert len(result) == 2
    assert result[0].id == 1
    assert result[1].id == 2


def test_get_records_failure(mock_get_records):
    mock_get_records(status_code=500)
    with raises(requests.HTTPError):
        sandbox_client.get_records()

    mock_get_records([{"unexpected": "response"}])
    with raises(pydantic.ValidationError):
        sandbox_client.get_records()


# get_record


def test_get_record_success(mock_get_record):
    mock = mock_get_record()

    result = sandbox_client.get_record(123)

    assert_headers_correct(mock)
    assert result.id == 123


def test_get_record_failure(mock_get_record):
    mock_get_record(status_code=500)
    with raises(requests.HTTPError):
        sandbox_client.get_record(123)

    mock_get_record({"unexpected": "response"})
    with raises(pydantic.ValidationError):
        sandbox_client.get_record(123)


# create


def test_create_success(mock_create):
    metadata = example_metadata()
    mock = mock_create()

    result = sandbox_client.create(metadata)

    assert_headers_correct(mock)
    assert result.id == 123
    assert mock.last_request.json()["metadata"] == metadata.model_dump()


def test_create_failure(mock_create):
    metadata = example_metadata()
    mock_create(status_code=400)
    with raises(requests.HTTPError):
        sandbox_client.create(metadata)

    mock_create({"unexpected": "response"})
    with raises(pydantic.ValidationError):
        sandbox_client.create(metadata)


# make_editable


@mark.parametrize("state", ["inprogress", "unsubmitted"])
def test_make_editable_success_when_editable(mock_make_editable, state):
    deposition = example_record(state=state)
    mock = mock_make_editable()

    result = sandbox_client.make_editable(deposition)

    assert not mock.called
    assert result.id == deposition.id


def test_make_editable_success_when_not_editable(mock_make_editable):
    deposition = example_record(state="done")
    mock = mock_make_editable()

    result = sandbox_client.make_editable(deposition)

    assert_headers_correct(mock)
    assert result.id == deposition.id


def test_make_editable_failure(mock_make_editable):
    deposition = example_record()
    mock_make_editable(status_code=400)
    with raises(requests.HTTPError):
        sandbox_client.make_editable(deposition)

    mock_make_editable({"unexpected": "response"})
    with raises(pydantic.ValidationError):
        sandbox_client.make_editable(deposition)


# discard


@mark.parametrize("state", ["inprogress", "unsubmitted"])
def test_discard_success_when_editable(mock_discard, state):
    mock = mock_discard()

    sandbox_client.discard(example_record(state=state))

    assert_headers_correct(mock)


def test_discard_success_when_not_editable(mock_discard):
    mock = mock_discard()

    sandbox_client.discard(example_record(state="done"))

    assert not mock.called


def test_discard_failure(mock_discard):
    mock_discard(status_code=400)
    with raises(requests.HTTPError):
        sandbox_client.discard(example_record(state="inprogress"))


# new_version


def test_new_version_success(mock_discard, mock_new_version):
    mock_discard()
    new_version_response = example_record(id=88)
    mock_new_version = mock_new_version(new_version_response.model_dump())

    result = sandbox_client.new_version(example_record(submitted=True))

    assert_headers_correct(mock_new_version)
    assert result.id == new_version_response.id


def test_new_version_flags_non_published(mock_discard, mock_new_version):
    mock_discard()
    mock_new_version()
    with raises(ValueError):
        sandbox_client.new_version(example_record(submitted=False))


def test_new_version_api_failure(mock_discard, mock_new_version):
    mock_discard()

    mock_new_version(status_code=400)
    with raises(requests.HTTPError):
        sandbox_client.new_version(example_record())

    mock_new_version({"unexpected": "response"})
    with raises(pydantic.ValidationError):
        sandbox_client.new_version(example_record())


# upload_file


def test_upload_file_success(mock_upload_file, tmp_path):
    file_path = tmp_path / "data.txt"
    file_path.write_text("This is my file.")
    deposition = example_record(submitted=False, state="unsubmitted")
    mock = mock_upload_file()

    result = sandbox_client.upload_file(deposition, file_path=file_path)

    assert_headers_correct(mock)
    assert mock.last_request.body.name == str(file_path)
    assert result


def test_upload_file_failure_api(mock_upload_file, tmp_path):
    file_path = tmp_path / "data.txt"
    file_path.write_text("This is my file.")
    deposition = example_record(submitted=False, state="unsubmitted")
    mock_upload_file(status_code=400)
    with raises(requests.HTTPError):
        sandbox_client.upload_file(deposition, file_path=file_path)

    mock_upload_file(json=[{"unexpected": "response"}])
    with raises(pydantic.ValidationError):
        sandbox_client.upload_file(deposition, file_path=file_path)


def test_upload_file_failure_file_not_found():
    with raises(FileNotFoundError):
        sandbox_client.upload_file(
            example_record(submitted=False, state="unsubmitted"),
            file_path=Path("data.txt"),
        )


def test_upload_file_failure_published():
    with raises(ValueError):
        sandbox_client.upload_file(
            example_record(submitted=True),
            file_path=Path("data.txt"),
        )


def test_upload_file_failure_no_bucket():
    with raises(ValueError):
        sandbox_client.upload_file(
            example_record(submitted=False, state="unsubmitted", bucket=None),
            file_path=Path("data.txt"),
        )


# publish


@mark.parametrize(
    "record",
    [
        example_record(submitted=False, state="unsubmitted"),
        example_record(submitted=True, state="inprogress"),
    ],
)
def test_publish_success_unpublished(mock_publish, record):
    mock = mock_publish()

    result = sandbox_client.publish(record)

    assert_headers_correct(mock)
    assert result.id == 123


def test_publish_success_published(mock_publish):
    mock = mock_publish()

    result = sandbox_client.publish(example_record(submitted=True, state="done"))

    assert not mock.called
    assert result.id == 123


def test_publish_failure(mock_publish):
    record = example_record(submitted=True, state="inprogress")
    mock_publish(status_code=500)
    with raises(requests.HTTPError):
        sandbox_client.publish(record)

    mock_publish({"unexpected": "response"})
    with raises(pydantic.ValidationError):
        sandbox_client.publish(record)
