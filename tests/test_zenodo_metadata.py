import tomlkit
from pytest import mark, raises

from zen_do.examples import example_metadata
from zen_do.internals import _write_metadata
from zen_do.zenodo_get_deposit import zenodo_get_deposit
from zen_do.zenodo_metadata import ZenodoRelatedIdentifier


def test_raises_error_if_zenodo_metadata_has_no_urn_id(monkeypatch, tmp_path):
    metadata = example_metadata()
    monkeypatch.chdir(tmp_path)
    del metadata.related_identifiers[0]
    _write_metadata(metadata)

    with raises(ValueError):
        zenodo_get_deposit([])


def test_raises_error_if_zenodo_metadata_has_multiple_urn_ids(monkeypatch, tmp_path):
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
    _write_metadata(metadata)

    with raises(ValueError):
        zenodo_get_deposit([])


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
    metadata = example_metadata().model_dump()
    metadata["related_identifiers"][0]["identifier"] = urn
    monkeypatch.chdir(tmp_path)
    (tmp_path / ".zenodo.toml").write_text(tomlkit.dumps(metadata))

    with raises(ValueError):
        zenodo_get_deposit([])
