use crate::metadata::{Creator, Metadata, RelatedIdentifier, RelatedIdentifierRelation};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, error::Error, fs, path::Path};

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
    pub relation: RelatedIdentifierRelation,
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
    pub extra: BTreeMap<String, serde_json::Value>,
}

impl From<&Metadata> for ZenodoJson {
    fn from(metadata: &Metadata) -> Self {
        let zendo_id = JsonRelatedIdentifier {
            identifier: metadata.zendo_id.to_urn(),
            relation: RelatedIdentifierRelation::IsIdenticalTo,
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
            extra: BTreeMap::new(),
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

/// Represents an author in the `CITATION.cff` file format.
#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct CffAuthor {
    pub given_names: String,
    pub family_names: String,
    pub affiliation: String,
    /// The ORCID of the author in the form of a URL.
    pub orcid: String,
}

impl From<&Creator> for CffAuthor {
    fn from(creator: &Creator) -> Self {
        Self {
            // TODO: Not sure how to handle the name
            given_names: creator.name.clone(),
            family_names: creator.name.clone(),
            affiliation: creator.affiliation.clone(),
            orcid: format!("https://orcid.org/{}", creator.orcid),
        }
    }
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CffIdentifierType {
    Doi,
    Url,
    Swh,
    #[default]
    Other,
}

/// Represents an identifier in the `CITATION.cff` file format.
/// As of v1.2.0, only identifiers with an `isIdenticalTo` relation are
/// supported.
#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct CffIdentifier {
    pub value: String,
    #[serde(rename = "type")]
    pub identifier_type: CffIdentifierType,
}

impl From<Option<&str>> for CffIdentifierType {
    fn from(scheme: Option<&str>) -> Self {
        match scheme {
            Some("doi") => Self::Doi,
            Some("url") => Self::Url,
            Some("swh") => Self::Swh,
            _ => Self::Other,
        }
    }
}

/// Represents the `CITATION.cff` file format.
#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct CitationCff {
    pub title: String,
    pub authors: Vec<CffAuthor>,
    pub identifiers: Vec<CffIdentifier>,

    // Any other fields in CITATION.cff
    #[serde(flatten)]
    pub extra: BTreeMap<String, serde_yaml::Value>,
}

impl From<&Metadata> for CitationCff {
    fn from(metadata: &Metadata) -> Self {
        let zendo_id = CffIdentifier {
            value: metadata.zendo_id.to_urn(),
            identifier_type: CffIdentifierType::Other,
        };
        let mut identifiers = metadata
            .related_identifiers
            .iter()
            .filter(|rel_id| rel_id.relation == RelatedIdentifierRelation::IsIdenticalTo)
            .map(|rel_id| CffIdentifier {
                value: rel_id.identifier.clone(),
                identifier_type: rel_id.scheme.as_deref().into(),
            })
            .collect::<Vec<_>>();
        identifiers.push(zendo_id);
        Self {
            title: metadata.title.clone(),
            authors: metadata.creators.iter().map(CffAuthor::from).collect(),
            identifiers,
            extra: BTreeMap::new(),
        }
    }
}

impl FormatFileName for CitationCff {
    const FILE_NAME: &str = "CITATION.cff";
}

impl Read for CitationCff {
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

        Ok(Some(serde_yaml::from_str(&contents)?))
    }
}

impl LeftJoin for CitationCff {
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

impl Write for CitationCff {
    fn write(self, parent: &Path) -> Result<(), Box<dyn Error>> {
        let path = parent.join(Self::FILE_NAME);
        fs::write(path, serde_yaml::to_string(&self)?)?;
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
    let new_metadata = T::from(metadata);
    let old_metadata = T::read(parent)?;
    let synced_metadata = new_metadata.left_join(old_metadata);
    synced_metadata.write(parent)?;
    Ok(())
}
