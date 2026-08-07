from pathlib import Path
from typing import Any

import tomlkit

from zen_do.zenodo_metadata import ZenodoMetadata


def _read_toml(path: Path) -> dict[str, Any]:
    return tomlkit.parse(path.read_text())


def _read_metadata(path: Path = Path(".zenodo.toml")) -> ZenodoMetadata:
    toml_file = _read_toml(path)
    return ZenodoMetadata.model_validate(toml_file)


def _write_metadata(
    metadata: ZenodoMetadata, path: Path = Path(".zenodo.toml")
) -> None:
    toml_text = tomlkit.dumps(metadata.model_dump_toml())
    path.write_text(toml_text)
