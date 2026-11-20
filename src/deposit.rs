use serde::Deserialize;

// TODO: `created` and `modified` might be useful info for list. We could
// include them later.
// TODO: I've also not kept `record_id`, `record_url`, and `owner` to keep this
// simple. Do we agree?
// TODO: It's not clear to me whether we still need to keep a `zendo_id` in this
// struct or it's fine to keep in related_identifiers. Not included for now.
/// A deposit as returned by the Zenodo API.
///
/// Only holds the fields zen-do uses. Any other fields in the response from
/// Zenodo are ignored when deserialising.
#[derive(Debug, Deserialize)]
pub struct Deposit {
    /// Deposit identifier.
    pub id: u64,

    /// Digital Object Identifier, created by Zenodo.
    pub doi: Option<String>,

    /// State of the deposit.
    pub state: DepositState,

    /// Submission status. True if the deposit has been published. False
    /// otherwise.
    pub submitted: bool,

    /// Vector of deposit files.
    pub files: Vec<DepositFile>,

    /// Deposit metadata.
    pub metadata: DepositMetadata,

    /// Deposit links.
    pub links: DepositLinks,
}

// TODO: Have not included filesize and checksum bc I don't think we need them.
// Agree?
/// A file attached to a deposit on Zenodo.
#[derive(Debug, Deserialize)]
pub struct DepositFile {
    /// Deposition file identifier.
    pub id: String,

    /// Name of the file.
    pub filename: String,
}

/// The deposit metadata as returned by Zenodo.
///
/// Not the same as `metadata::Metadata`, which holds what the user writes in the
/// `.zenodo.toml`. Fields are optional here because deposits that weren't made
/// by zen-do might not have them (e.g., empty drafts).
#[derive(Debug, Deserialize)]
pub struct DepositMetadata {
    /// Title of the deposit.
    pub title: Option<String>,

    /// Abstract or description of the deposit (allows HTML).
    pub description: Option<String>,

    /// Persistent identifiers of related publications and datasets.
    #[serde(default)] // If Zenodo leaves this field out, use an empty list instead of failing.
    pub related_identifiers: Vec<DepositRelatedIdentifier>,
}

/// An identifier that links the deposit to another piece of work, e.g., a paper
/// or another deposit.
///
/// Kept separate from `metadata::RelatedIdentifier`, which rejects the
/// `urn:zendo:` identifier that is stored here on Zenodo.
#[derive(Debug, Deserialize)]
pub struct DepositRelatedIdentifier {
    /// Persistent identifier.
    pub identifier: String,

    /// Relationship to the deposit.
    pub relation: String,
}

/// The processing state of a deposit.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DepositState {
    InProgress,
    Done,
    Error,
    Unsubmitted, // Not in Deposit doc, but shown in the Quickstart example.
}

/// Links Zenodo includes with a deposit. Only keeping the ones zen-do needs.
#[derive(Debug, Deserialize)]
pub struct DepositLinks {
    /// URL to upload files to.
    pub bucket: Option<String>,

    /// URL of the newest draft of the deposit. Used after a new version has
    /// been created, because the Zenodo response returns the old version,
    /// not the new draft.
    pub latest_draft: Option<String>,
}
