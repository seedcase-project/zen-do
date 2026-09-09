// TODO: Add module documentation.

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

pub const EXAMPLE_METADATA: &str = r#"
title = "Random"
upload_type = "random"

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
// TODO: Include urn property? As in the Python?
// type for it?
/// Contains representing Zenodo metadata.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Metadata {
    /// The title of the deposit.
    pub title: String,

    // TODO: Create an UploadType enum.
    /// The type of the deposit.
    pub upload_type: String,

    // TODO: Don't allow empty vec, NonEmptyVec?
    /// The creators of the deposit.
    pub creators: Vec<Creator>,

    /// Identifiers related to the deposit.
    pub related_identifiers: Vec<RelatedIdentifier>,
}

/// The type containing the details of the creator/author of a Zenodo deposit.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Creator {
    /// The full name of the creator/author.
    pub name: String,

    /// The (primary) affiliation of the creator/author.
    pub affiliation: String,

    /// The ORCID of the creator/author.
    pub orcid: String,
}

// TODO: Create a check for our URN id, `urn:zenodo:*`, maybe by making a
// specific type for it?
/// Model representing an identifier related to a Zenodo deposit.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
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
/// Outputs a `Box` of Errors if the file couldn't be read correctly or if the
/// TOML couldn't be parsed.
pub fn read_metadata(path: &Path) -> Result<Metadata, Box<dyn Error>> {
    // `&Path` is a borrowed immutable reference, since it points to where the file
    // lives.

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
/// Errors to writing to file, such as if there is a problem with the file
/// itself or where it will be saved in.
pub fn write_metadata(metadata: &Metadata, path: PathBuf) -> Result<(), Box<dyn Error>> {
    // `PathBuf` is the owned path to the file, owned to ensure nothing else can
    // write to it at the same time.

    // TODO: May have to use another way to write, to preserve comments and order.
    // Maybe `toml_edit`?
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
        // TODO: Refactor to write to memory representation of writing, not actual
        // writing (less I/O in tests).
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
        let path = tempfile::NamedTempFile::new().unwrap().path().to_path_buf();

        let example: Metadata = toml::from_str(EXAMPLE_METADATA).unwrap();
        let write_result = write_metadata(&example, path);

        assert!(write_result.is_ok());
    }
}
