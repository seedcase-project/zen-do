// TODO: Add module documentation.

use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::error::Error;
use std::fs;
use std::path::Path;

pub const EXAMPLE_METADATA: &str = r#"
title = "Random"
upload_type = "random"

# This is a required ID for zen-do
zendo_id = "my-org:project:book"

[[creators]]
name = "Tip Top"
affiliation = "University"
orcid = "12345"

[[related_identifiers]]
identifier = "random"
relation = "isIdenticalTo"
resource_type = "test"
"#;

// TODO: Include a check that the URNs are unique, maybe by making a specific
// type for it?

// Create deserialize check for the zen-do urn:
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(try_from = "String")]
pub struct ZendoId(String);

impl TryFrom<String> for ZendoId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let components: Vec<&str> = value.split(':').collect();

        if components.len() < 2 || components.iter().any(|part| part.is_empty()) {
            return Err(
                "The field `zendo_id` in the Zenodo TOML file must have at least two parts separated by a ´:´ (an example is given when using the `init` command).".into(),
            );
        }

        if components
            .iter()
            .any(|part| matches!(*part, "<github-org>" | "<output-type>"))
        {
            return Err("The field `zendo_id` in the Zenodo TOML file cannot contain placeholder values (e.g., `<github-org>`). Please fill them in with actual values.".into());
        }

        let correct_characters = components.iter().all(|part| {
            part.chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.')
        });

        if !correct_characters {
            return Err("The field `zendo_id` in the Zenodo TOML file can only contain alphanumeric characters in addition to `-`, `_`, `.`. It will also contain `:` as a separator.".into());
        }

        Ok(Self(value))
    }
}

/// Type representing Zenodo metadata.
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Metadata {
    /// The title of the deposit.
    pub title: String,

    /// Optional version label of the deposit. Any string is accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    // TODO: Create an UploadType enum.
    /// The type of the deposit.
    pub upload_type: String,

    /// The zen-do identifier that identifies whether a deposit can be tracked
    /// by zen-do.
    pub zendo_id: ZendoId,

    // TODO: Don't allow empty vec, NonEmptyVec?
    /// The creators of the deposit.
    pub creators: Vec<Creator>,

    /// Identifiers related to the deposit.
    pub related_identifiers: Vec<RelatedIdentifier>,
}

impl ZendoId {
    const PREFIX: &str = "urn:zendo:";
    // Only used for writing with init - compiler will not read a placeholder
    #[must_use]
    pub fn placeholder(project_name: &str) -> Self {
        Self(format!("<github-org>:{project_name}:<output-type>"))
    }
    // Method that returns a reference to the inner string without copying it.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn to_urn(&self) -> String {
        format!("{}{}", Self::PREFIX, self.0)
    }
}

impl Metadata {
    #[must_use]
    pub fn new(project_name: &str) -> Self {
        Self {
            zendo_id: ZendoId::placeholder(project_name),
            creators: vec![Creator::default()],
            related_identifiers: vec![RelatedIdentifier::default()],
            ..Default::default()
        }
    }
}

/// The type containing the details of the creator/author of a Zenodo deposit.
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Creator {
    /// The full name of the creator/author.
    pub name: String,

    /// The (primary) affiliation of the creator/author.
    pub affiliation: String,

    /// The ORCID of the creator/author.
    pub orcid: String,
}

/// Model representing an identifier related to a Zenodo deposit.
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(try_from = "RelatedIdentifierCheck")]
pub struct RelatedIdentifier {
    /// The value of the identifier (meaning, the identifier itself).
    pub identifier: String,

    /// The relationship between the deposit and the other piece of work
    /// identified by the identifier.
    pub relation: RelatedIdentifierRelation,

    // TODO: Create a ResourceType enum.
    /// The type of the work identified by the identifier.
    pub resource_type: String,

    /// The scheme followed by the identifier.
    pub scheme: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub enum RelatedIdentifierRelation {
    IsCitedBy,
    Cites,
    IsSupplementTo,
    IsSupplementedBy,
    IsContinuedBy,
    Continues,
    IsDescribedBy,
    Describes,
    HasMetadata,
    IsMetadataFor,
    HasVersion,
    IsVersionOf,
    IsNewVersionOf,
    IsPreviousVersionOf,
    IsPartOf,
    HasPart,
    IsPublishedIn,
    IsReferencedBy,
    References,
    IsDocumentedBy,
    Documents,
    IsCompiledBy,
    Compiles,
    IsVariantFormOf,
    IsOriginalFormOf,
    IsIdenticalTo,
    IsAlternateIdentifier,
    IsReviewedBy,
    Reviews,
    IsDerivedFrom,
    #[default]
    IsSourceOf,
    Requires,
    IsRequiredBy,
    IsObsoletedBy,
    Obsoletes,
}

#[derive(Debug, Deserialize)]
struct RelatedIdentifierCheck {
    pub identifier: String,
    pub relation: RelatedIdentifierRelation,
    pub resource_type: String,
    pub scheme: Option<String>,
}

impl TryFrom<RelatedIdentifierCheck> for RelatedIdentifier {
    type Error = String;

    fn try_from(related_identifier: RelatedIdentifierCheck) -> Result<Self, Self::Error> {
        if related_identifier.identifier.starts_with(ZendoId::PREFIX)
            && related_identifier.relation == RelatedIdentifierRelation::IsIdenticalTo
        {
            return Err(
                "Manual setup of zen-do URN is not allowed. Please use the field `zendo_id` (an example is given when using the `init` command)."
                    .into(),
            );
        }

        Ok(Self {
            identifier: related_identifier.identifier,
            relation: related_identifier.relation,
            resource_type: related_identifier.resource_type,
            scheme: related_identifier.scheme,
        })
    }
}

// `Box<>` is a container to hold some unknown type of objects. It allocates on
// the heap, so we don't want to use this often, but reading is a good place for
// it.

// `dyn` is added by Rust analyzer/formatter, which is dynamically dispatched.
// The program can't determine the exact error type until runtime.

// TODO: Should this be `read_toml`? :thinking:

/// Reads the Zenodo TOML metadata file
///
/// # Arguments
///
/// - `path`: This is the path to the TOML file.
///
/// # Errors
///
/// Outputs a `Box` containing an error if the file couldn't be read correctly
/// or if the TOML couldn't be parsed.
pub fn read_metadata(path: &Path) -> Result<Metadata, Box<dyn Error>> {
    // `&Path` is a borrowed immutable reference to a file on the system.

    // `?` means to grab any error types and output them as the `Result`.
    let content: String = fs::read_to_string(path)?;
    let metadata: Metadata = toml::from_str(&content)?;
    Ok(metadata)
}

/// Writes the Zenodo metadata to the TOML file.
///
/// # Arguments
///
/// - `metadata`: The `Metadata` struct that will be converted to TOML and saved
///   to the `path`.
/// - `path`: The path to the file to save the `metadata`.
///
/// # Errors
///
/// Errors when writing to file, such as if there is a problem with the file
/// itself or where it will be saved.
pub fn write_metadata(metadata: &Metadata, path: &Path) -> Result<(), Box<dyn Error>> {
    let toml_str: String = toml::to_string_pretty(metadata)?;
    fs::write(path, toml_str)?;
    Ok(())
}

/// Sets the version in an existing Zenodo TOML metadata file.
///
/// Preserves other fields, comments, and formatting. The file only needs to
/// be valid TOML; it does not need to contain complete deposit metadata.
///
/// # Arguments
///
/// - `version`: The version label to set, without restricting its format.
/// - `path`: The path to the TOML metadata file.
///
/// # Errors
///
/// Returns an error if the file cannot be read, parsed as TOML, or written.
pub fn set_version(version: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let mut document = contents.parse::<toml_edit::DocumentMut>()?;
    let mut version_value = toml_edit::Value::from(version);

    if let Some(existing) = document.get("version").and_then(toml_edit::Item::as_value) {
        *version_value.decor_mut() = existing.decor().clone();
    }

    document["version"] = toml_edit::Item::Value(version_value);
    fs::write(path, document.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    // To import all code from above in this file.
    use super::*;

    #[test]
    fn test_parse_example() {
        let metadata: Result<Metadata, _> = toml::from_str(EXAMPLE_METADATA);
        // Uncomment to debug during testing.
        // println!("{:?}", metadata);
        assert!(metadata.is_ok())
    }

    #[test]
    fn test_reading_metadata() {
        // TODO: Refactor to write to memory representation of writing, not
        // actual writing (less I/O in tests)?
        use std::io::Write;

        // `mut` since the file will be written to.
        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(EXAMPLE_METADATA.as_bytes()).unwrap();

        let path = file.path().to_path_buf();
        let metadata = read_metadata(&path).unwrap();
        let expected: Metadata = toml::from_str(EXAMPLE_METADATA).unwrap();

        // Compare all because of `PartialEq`.
        assert_eq!(metadata, expected);
    }

    #[test]
    fn test_writing_metadata() {
        // Rust suggested using this approach as it allows `path` to last longer
        // as a value.  Since `binding` is an owned value, using `path()` on it
        // allows the reference back to it.  See `rustc --explain E0716`
        let binding = tempfile::NamedTempFile::new().unwrap();
        let path = binding.path();

        let example: Metadata = toml::from_str(EXAMPLE_METADATA).unwrap();
        let write_result = write_metadata(&example, path);
        let actual = read_metadata(path);

        assert!(write_result.is_ok());
        assert_eq!(example, actual.unwrap());
    }

    #[test]
    fn set_version_preserves_other_fields_and_comments() {
        let temp_dir = tempfile::tempdir().unwrap();
        let metadata_path = temp_dir.path().join(".zenodo.toml");
        let original = "# Metadata\nversion = \"old\" # Release label\nlicense = \"mit\"\n";
        fs::write(&metadata_path, original).unwrap();

        set_version("1.2.3", &metadata_path).unwrap();

        assert_eq!(
            fs::read_to_string(&metadata_path).unwrap(),
            "# Metadata\nversion = \"1.2.3\" # Release label\nlicense = \"mit\"\n"
        );
    }

    #[test]
    fn set_version_adds_a_root_field() {
        let temp_dir = tempfile::tempdir().unwrap();
        let metadata_path = temp_dir.path().join(".zenodo.toml");
        fs::write(&metadata_path, "[custom]\nversion = \"nested\"\n").unwrap();

        set_version("nightly build", &metadata_path).unwrap();

        let contents = fs::read_to_string(&metadata_path).unwrap();
        let updated: toml::Value = toml::from_str(&contents).unwrap();
        assert_eq!(updated["version"].as_str(), Some("nightly build"));
        assert_eq!(updated["custom"]["version"].as_str(), Some("nested"));
    }

    #[test]
    fn set_version_leaves_invalid_toml_untouched() {
        let temp_dir = tempfile::tempdir().unwrap();
        let metadata_path = temp_dir.path().join(".zenodo.toml");
        let original = "title = \"unterminated\n";
        fs::write(&metadata_path, original).unwrap();

        assert!(set_version("1.2.3", &metadata_path).is_err());
        assert_eq!(fs::read_to_string(&metadata_path).unwrap(), original);
    }

    #[test]
    fn test_get_zendo_id() {
        let temp_dir = tempfile::tempdir().unwrap();
        let metadata_path = temp_dir.path().join(".zenodo.toml");

        std::fs::write(
            &metadata_path,
            r#"
        title = ""
        upload_type = ""
        zendo_id = "my-org:my-project"
        creators = []
        related_identifiers = []
        "#,
        )
        .unwrap();

        let metadata = read_metadata(metadata_path.as_path()).unwrap();

        let urn = metadata.zendo_id.as_str();

        assert_eq!(urn, "my-org:my-project");
    }

    #[test]
    fn test_zendo_id_character_space() {
        let temp_dir = tempfile::tempdir().unwrap();
        let metadata_path = temp_dir.path().join(".zenodo.toml");

        std::fs::write(
            &metadata_path,
            r#"
        title = ""
        upload_type = ""
        zendo_id = "my/org:my project"
        creators = []
        related_identifiers = []
        "#,
        )
        .unwrap();

        assert!(read_metadata(metadata_path.as_path()).is_err());
    }

    #[test]
    fn test_user_manually_setting_zendo_urn_related_identifier() {
        let temp_dir = tempfile::tempdir().unwrap();
        let metadata_path = temp_dir.path().join(".zenodo.toml");

        std::fs::write(
            &metadata_path,
            r#"
        title = ""
        upload_type = ""
        zendo_id = "my-org:my project"
        creators = []
        related_identifiers = [
            { identifier = "urn:zendo:my-org:my-project", relation = "isIdenticalTo", resource_type = "software", scheme = "urn" }
        ]
        "#,
        )
        .unwrap();

        let err = read_metadata(metadata_path.as_path()).unwrap_err();
        assert!(
            err.to_string().contains(
                "Manual setup of zen-do URN is not allowed. Please use the field `zendo_id`"
            ),
            "Unexpected error: {err}"
        );
    }
}
