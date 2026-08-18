# class ZenodoRelatedIdentifier(KebabModel, frozen=True):
#     @model_validator(mode="after")
#     def _check_urn(self) -> Self:

#         if self.scheme == "urn" and not re.fullmatch(
#             r"urn:zenodo(:[^/:]+)+", self.identifier
#         ):
#             raise ValueError(
#                 f"The URN {self.identifier!r} does not have the expected format. URNs "
#                 "must be in the format 'urn:zenodo:<unique-id>(:<optional-sub-id>)'. "
#                 "We recommend 'urn:zenodo:<github-username>:<repo-name>:<output-type>'."
#             )
#         return self

# class ZenodoMetadata(KebabModel, frozen=True):
#     @property
#     def urn(self) -> str:
#         """The URN related identifier of the deposit."""
#         urns = so.keep(self.related_identifiers, _is_urn)
#         return urns[0].identifier

#     @model_validator(mode="after")
#     def _check_unique_urn(self) -> Self:
#         urns = so.keep(self.related_identifiers, _is_urn)
#         if len(urns) != 1:
#             raise ValueError(
#                 "Expected exactly one `isIdenticalTo` URN in the Zenodo metadata file "
#                 f"under `related_identifiers`, but found {len(urns)}. Ensure there is "
#                 "a single unique URN, as it is used to identify the corresponding "
#                 "deposit on Zenodo."
#             )
#         return self

# def _is_urn(id: ZenodoRelatedIdentifier) -> bool:
#     return id.relation == "isIdenticalTo" and id.scheme == "urn"
