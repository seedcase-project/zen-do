// TODO: Add module documentation.

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::{Path, PathBuf};
use std::fs;

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

# This is a required ID for zen-do
[[related_identifiers]]
identifier = "urn:zenodo:my-org:project:book"
relation = "isIdenticalTo"
resource_type="other"
scheme="urn"
"#;

// TODO: Include a check that the URNs are unique, maybe by making a specific
// type for it?

// TODO: Include urn property? As in the Python?

/// Type representing Zenodo metadata.
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

/// Create a `.zenodo.toml` file with all deposit metadata fields
///
/// # Arguments
///
/// - `project_cd`: The current directory of the terminal
/// - `verbose`: Write feedback in the terminal
///
/// # Errors
///
/// Errors when writing to file, such as if there is a problem with the file
/// itself or where it will be saved.
pub fn init(project_cd: PathBuf) -> Result<(), Box<dyn Error>> {
    let metadata_path = project_cd.join(".zenodo.toml");

    // Should be an initArg??
    let verbose = true;

    if metadata_path.is_file() {
        if verbose {
            println!("A `.zenodo.toml` file already exists in this directory.");
        }
        return Ok(());
    }

    // Makes it difficult to do a good unit test... Changing dir with parallel test execution
    // now in mainlet project_cd = env::current_dir()?;

    let project_name = project_cd
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("Current directory contains unsupported characters")?;

    let metadata = Metadata {
        title: String::new(),
        upload_type: String::new(),
        creators: vec![Creator {
            name: String::new(),
            affiliation: String::new(),
            orcid: String::new(),
        }],
        related_identifiers: vec![RelatedIdentifier {
            identifier: format!("urn:zenodo:<github-org>:{project_name}"),
            relation: "isIdenticalTo".to_string(),
            resource_type: "other".to_string(),
            scheme: Some("urn".to_string()),
        }],
    };

    write_metadata(&metadata, &metadata_path)?;

    if verbose {
        print!("Created an empty `.zenodo.toml` file.");
    }

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
    fn test_init_command_no_prior_file() {

        let temp_dir = tempfile::tempdir().unwrap();

        init(temp_dir.path().to_path_buf()).unwrap();

        let metadata_file = temp_dir.path().join(".zenodo.toml");

        // 1. Was the file created?
        assert!(metadata_file.is_file());

        // 2. Can read_metadata parse it?
        let metadata = read_metadata(&metadata_file).unwrap();

        assert_eq!(metadata.title, "");
        assert_eq!(metadata.upload_type, "");
        assert_eq!(metadata.creators.len(), 1);
    }
}
