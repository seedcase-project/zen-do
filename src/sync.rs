use crate::metadata::{Creator, Metadata, RelatedIdentifier};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs, path::Path};

/// Provides the filename used to store the format.
pub trait FormatFileName {
    /// The filename used to store the format.
    const FILE_NAME: &str;
}

/// Provides functionality for reading a format from disk.
pub trait Read: Sized {
    /// Reads the format from disk.
    ///
    /// Returns `Ok(None)` if the file does not exist.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or deserialised.
    fn read(parent: &Path) -> Result<Option<Self>, Box<dyn Error>>;
}

/// Provides functionality for merging two instances of a format.
pub trait LeftJoin: Sized {
    /// Merges two instances of a format, keeping the values from `self`
    /// and filling in additional fields from `right`, if present. Arrays and
    /// nested objects in `self` are retained completely rather than being
    /// merged with their corresponding fields in `right`.
    #[must_use]
    fn left_join(self, right: Option<Self>) -> Self;
}

/// Provides functionality for writing a format to disk.
pub trait Write {
    /// Writes the format to disk.
    ///
    /// # Errors
    ///
    /// Returns an error if the format cannot be serialised or written to disk.
    fn write(self, parent: &Path) -> Result<(), Box<dyn Error>>;
}

/// Represents a creator in the `.zenodo.json` file format.
#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct JsonCreator {
    pub name: String,
    pub affiliation: String,
    pub orcid: String,
}

impl From<&Creator> for JsonCreator {
    fn from(creator: &Creator) -> Self {
        Self {
            name: creator.name.clone(),
            affiliation: creator.affiliation.clone(),
            orcid: creator.orcid.clone(),
        }
    }
}

/// Represents a related identifier in the `.zenodo.json` file format.
#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct JsonRelatedIdentifier {
    pub identifier: String,
    pub relation: String,
    pub resource_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
}

impl From<&RelatedIdentifier> for JsonRelatedIdentifier {
    fn from(related_identifier: &RelatedIdentifier) -> Self {
        Self {
            identifier: related_identifier.identifier.clone(),
            relation: related_identifier.relation.clone(),
            resource_type: related_identifier.resource_type.clone(),
            scheme: related_identifier.scheme.clone(),
        }
    }
}

/// Represents the `.zenodo.json` file format.
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ZenodoJson {
    pub title: String,
    pub upload_type: String,
    pub creators: Vec<JsonCreator>,
    pub related_identifiers: Vec<JsonRelatedIdentifier>,

    // Any other fields in .zenodo.json
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl From<&Metadata> for ZenodoJson {
    fn from(metadata: &Metadata) -> Self {
        let zendo_id = JsonRelatedIdentifier {
            identifier: metadata.zendo_id.to_urn(),
            relation: "isIdenticalTo".into(),
            resource_type: metadata.upload_type.clone(),
            scheme: Some("urn".into()),
        };
        let mut related_identifiers = metadata
            .related_identifiers
            .iter()
            .map(JsonRelatedIdentifier::from)
            .collect::<Vec<_>>();
        related_identifiers.push(zendo_id);
        Self {
            title: metadata.title.clone(),
            upload_type: metadata.upload_type.clone(),
            creators: metadata.creators.iter().map(JsonCreator::from).collect(),
            related_identifiers,
            extra: HashMap::new(),
        }
    }
}

impl FormatFileName for ZenodoJson {
    const FILE_NAME: &str = ".zenodo.json";
}

impl Read for ZenodoJson {
    fn read(parent: &Path) -> Result<Option<Self>, Box<dyn Error>> {
        let path = parent.join(Self::FILE_NAME);
        let contents = match fs::read_to_string(path) {
            Ok(contents) => contents,
            // If the target file doesn't exist, there is nothing to join
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                return Ok(None);
            }
            Err(err) => return Err(err.into()),
        };

        Ok(Some(serde_json::from_str(&contents)?))
    }
}

impl LeftJoin for ZenodoJson {
    fn left_join(self, right: Option<Self>) -> Self {
        match right {
            Some(right) => Self {
                extra: right.extra,
                ..self
            },
            None => self,
        }
    }
}

impl Write for ZenodoJson {
    fn write(self, parent: &Path) -> Result<(), Box<dyn Error>> {
        let path = parent.join(Self::FILE_NAME);
        let mut contents = serde_json::to_string_pretty(&self)?;
        contents.push('\n');
        fs::write(path, contents)?;
        Ok(())
    }
}

/// Syncs the specified format with the metadata.
///
/// # Arguments
///
/// - `metadata`: The metadata to sync to the specified format.
/// - `parent`: The directory containing the metadata and format files.
///
/// # Errors
///
/// Returns an error if reading the existing format file or writing the
/// synced format file fails.
pub fn sync_format<T>(metadata: &Metadata, parent: &Path) -> Result<(), Box<dyn Error>>
where
    T: for<'a> From<&'a Metadata> + Read + LeftJoin + Write,
{
    let new_format = T::from(metadata);
    let old_format = T::read(parent)?;
    let synced_format = new_format.left_join(old_format);
    synced_format.write(parent)?;
    Ok(())
}
