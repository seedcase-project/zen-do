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
zendo_urn = "urn:zenodo:my-org:project:book"

[[creators]]
name = "Tip Top"
affiliation = "University"
orcid = "12345"

[[related_identifiers]]
identifier = "random"
relation = "link"
resource_type = "test"
"#;

// TODO: Include a check that the URNs are unique, maybe by making a specific
// type for it?

// Create deserialize check for the zen-do urn:
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(try_from = "String")]
pub struct ZendoUrn(String);

impl TryFrom<String> for ZendoUrn {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // white spaces
        let value = value.trim();

        let Some(identifier) = value.strip_prefix("urn:zenodo:") else {
            return Err("zenodo.toml field 'zendo_urn' must start with 'urn:zenodo:'".into());
        };

        let components: Vec<&str> = identifier.split(':').collect();

        if components.len() < 2 || components.iter().any(|part| part.is_empty()) {
            return Err(
                "zenodo.toml field 'zendo_urn' must contain non-empty identifier components".into(),
            );
        }

        if components
            .iter()
            .any(|part| matches!(*part, "<github-org>" | "<output-type>"))
        {
            return Err("zenodo.toml field 'zendo_urn' must not contain placeholder values (e.g., <github-org>)".into());
        }

        Ok(Self(value.to_owned()))
    }
}

// TODO: Include urn property? As in the Python?

/// Type representing Zenodo metadata.
#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Metadata {
    /// The title of the deposit.
    pub title: String,

    // TODO: Create an UploadType enum.
    /// The type of the deposit.
    pub upload_type: String,

    /// The zen-do URN
    pub zendo_urn: ZendoUrn,

    // TODO: Don't allow empty vec, NonEmptyVec?
    /// The creators of the deposit.
    pub creators: Vec<Creator>,

    /// Identifiers related to the deposit.
    pub related_identifiers: Vec<RelatedIdentifier>,
}

impl ZendoUrn {
    // Only used for writing with init - compiler will not read a placeholder
    #[must_use]
    pub fn placeholder(project_name: &str) -> Self {
        Self(format!(
            "urn:zenodo:<github-org>:{project_name}:<output-type>"
        ))
    }
    // Method that returns a reference to the inner string without copying it.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Reads the Zendo URN from metadata at the given path.
pub fn get_zendo_urn(path: &Path) -> Result<String, Box<dyn Error>> {
    let metadata = read_metadata(path)?;
    Ok(metadata.zendo_urn.as_str().to_string())
}

impl Metadata {
    #[must_use]
    pub fn new(project_name: &str) -> Self {
        Self {
            zendo_urn: ZendoUrn::placeholder(project_name),
            //zendo_urn: ZendoUrn::placeholder(urn.clone()),
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
pub struct RelatedIdentifier {
    /// The value of the identifier (meaning, the identifier itself).
    pub identifier: String,

    // TODO: Create a Relation enum.
    /// The relationship between the deposit and the other piece of work
    /// identified by the identifier.
    pub relation: String,

    // TODO: Create a ResourceType enum.
    /// The type of the work identified by the identifier.
    pub resource_type: String,

    /// The scheme followed by the identifier.
    pub scheme: Option<String>,
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
    fn test_get_zendo_urn() {
        let temp_dir = tempfile::tempdir().unwrap();
        let metadata_file = temp_dir.path().join(".zenodo.toml");

        std::fs::write(
            &metadata_file,
            r#"
        title = ""
        upload_type = ""
        zendo_urn = "urn:zenodo:my-org:my-project"
        creators = []
        related_identifiers = []
        "#,
        )
        .unwrap();

        let urn = get_zendo_urn(&metadata_file).unwrap();

        assert_eq!(urn, "urn:zenodo:my-org:my-project");
    }
}
