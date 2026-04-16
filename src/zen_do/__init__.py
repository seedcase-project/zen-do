"""Module containing all source code."""

from .cli import zenodo_publish
from .examples import example_deposit, example_metadata
from .zenodo import (
    ZenodoClient,
    ZenodoCreator,
    ZenodoDeposit,
    ZenodoDepositState,
    ZenodoLinks,
    ZenodoMetadata,
    ZenodoRelatedIdentifier,
)

__all__ = [
    "example_metadata",
    "example_deposit",
    "zenodo_publish",
    "ZenodoClient",
    "ZenodoCreator",
    "ZenodoLinks",
    "ZenodoMetadata",
    "ZenodoDeposit",
    "ZenodoDepositState",
    "ZenodoRelatedIdentifier",
]
