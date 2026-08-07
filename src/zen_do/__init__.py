"""Module containing all source code."""

from .cli import get, list
from .examples import example_deposit, example_metadata
from .zenodo_client import (
    ZenodoClient,
    ZenodoDepositState,
)
from .zenodo_metadata import ZenodoCreator, ZenodoMetadata, ZenodoRelatedIdentifier

__all__ = [
    "get",
    "example_metadata",
    "example_deposit",
    "list",
    "ZenodoClient",
    "ZenodoCreator",
    "ZenodoMetadata",
    "ZenodoDepositState",
    "ZenodoRelatedIdentifier",
]
