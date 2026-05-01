import re
import tomllib
from pathlib import Path

import toml

from zen_do.zenodo_metadata import ZenodoMetadata


def _read_metadata() -> ZenodoMetadata:
    with open(Path(".zenodo.toml"), mode="rb") as file:
        toml_file = tomllib.load(file)

    return ZenodoMetadata.model_validate(toml_file)


def _write_metadata(
    metadata: ZenodoMetadata, path: Path = Path(".zenodo.toml")
) -> None:
    toml_text = toml.dumps(metadata.model_dump())
    # Add newline before each header
    toml_text = re.sub(r"\n*\[\[", "\n\n[[", toml_text)
    path.write_text(toml_text)
