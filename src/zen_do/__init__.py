"""Module containing all source code."""

from .cli import zenodo_publish
from .examples import example_deposit, example_metadata
from .zenodo import (
    ZenodoClient,
    ZenodoCreator,
    ZenodoDepositState,
    ZenodoMetadata,
    ZenodoRelatedIdentifier,
)

__all__ = [
    "example_metadata",
    "example_deposit",
    "zenodo_publish",
    "ZenodoClient",
    "ZenodoCreator",
    "ZenodoMetadata",
    "ZenodoDepositState",
    "ZenodoRelatedIdentifier",
]
