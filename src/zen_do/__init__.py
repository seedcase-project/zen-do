"""Module containing all source code."""

from .cli import init, list
from .examples import example_deposit, example_metadata
from .zenodo_client import (
    ZenodoClient,
    ZenodoDepositState,
)
from .zenodo_metadata import ZenodoCreator, ZenodoMetadata, ZenodoRelatedIdentifier

__all__ = [
    "ZenodoClient",
    "ZenodoCreator",
    "ZenodoDepositState",
    "ZenodoMetadata",
    "ZenodoRelatedIdentifier",
    "example_deposit",
    "example_metadata",
    "init",
    "list",
]
