from dataclasses import dataclass
from enum import Enum
from pathlib import Path

import tomlkit

from zen_do.internals import _read_toml
from zen_do.zenodo_metadata import ZenodoMetadata


@dataclass
class ConvertedMetadata:
    """Class representing metadata that has been converted to a different format."""

    content: str
    output_path: Path


class Format(Enum):
    """Supported formats to convert Zenodo metadata to."""

    zenodo_json = ".zenodo.json"
    citation_cff = "CITATION.cff"
    pyproject_toml = "pyproject.toml"
    quarto_yml = "_quarto.yml"
    description = "DESCRIPTION"

    def convert(
        self, metadata: ZenodoMetadata, metadata_path: Path
    ) -> ConvertedMetadata:
        """Convert the given metadata to the format specified by this enum member."""
        match self:
            case Format.zenodo_json:
                return ConvertedMetadata(
                    content=metadata.model_dump_json(indent=2),
                    output_path=metadata_path.parent / f"{metadata_path.stem}.json",
                )
            case Format.pyproject_toml:
                pyproject_toml = _read_toml(Path("pyproject.toml"))
                tool = pyproject_toml.setdefault("tool", tomlkit.table())
                tool["zenodo"] = metadata.model_dump_toml()
                return ConvertedMetadata(
                    content=tomlkit.dumps(pyproject_toml),
                    output_path=Path("pyproject.toml"),
                )
            case _:
                raise NotImplementedError()
