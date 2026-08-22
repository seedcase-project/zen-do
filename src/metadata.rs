// TODO: Add module documentation.

use serde::Deserialize;

// TODO: Include a check that the URNs are unique, maybe by making a specific
// TODO: Include urn property? As in the Python?
// type for it?
/// Contains representing Zenodo metadata.
#[derive(Debug, Deserialize)]
pub struct Metadata {
    /// The title of the deposit.
    pub title: String,

    // TODO: Create an UploadType enum.
    /// The type of the deposit.
    pub upload_type: String,

    /// The creators of the deposit.
    pub creators: Vec<Creator>,

    /// Identifiers related to the deposit.
    pub related_identifiers: Vec<RelatedIdentifier>,
}

/// The type containing the details of the creator/author of a Zenodo deposit.
#[derive(Debug, Deserialize)]
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
#[derive(Debug, Deserialize)]
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
pub fn read_metadata(path: &str) -> Result<Metadata, Box<dyn std::error::Error>> {
    // `?` means to grab any error types and output them as the `Result`.
    let content: String = fs::read_to_string(path)?;
    let metadata: Metadata = toml::from_str(&content)?;
    Ok(metadata)
}

#[cfg(test)]
mod tests {
    // To import all code from above in this file.
    use super::*;

    #[test]
    fn test_fully_read() {
        let toml_str = r#"
title = "Random"
upload_type = "random"

[[creators]]
name = "Jim"
affiliation = "University"
orcid = "12345"

[[related_identifiers]]
identifier = "random"
relation = "link"
resource_type = "test"
    "#;

        let metadata: Result<Metadata, _> = toml::from_str(toml_str);
        println!("{:?}", metadata);
        assert!(metadata.is_ok())
    }
}
