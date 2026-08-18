// TODO: Add module documentation.

use serde::Deserialize;
use std::fs;

// TODO: Include a check that the URNs are unique, maybe by making a specific type for it?
/// Contains representing Zenodo metadata.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Metadata {
    /// The title of the deposit.
    title: String,

    // TODO: Create an UploadType enum.
    /// The type of the deposit.
    upload_type: String,

    /// The creators of the deposit.
    creators: Vec<Creator>,

    /// Identifiers related to the deposit.
    related_identifiers: Vec<RelatedIdentifier>

    // TODO: Include urn property? As in the Python?
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

// TODO: Create a check for our URN id, `urn:zenodo:*`, maybe by making a specific type for it?
/// Model representing an identifier related to a Zenodo deposit.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RelatedIdentifier {
    /// The value of the identifier (meaning, the identifier itself).
    identifier: String,

    // TODO: Create a Relation enum.
    /// The relationship between the deposit and the other piece of work identified by the identifier.
    relation: String,

    // TODO: Create a ResourceType enum.
    /// The type of the work identified by the identifier.
    resource_type: String,

    /// The scheme followed by the identifier.
    scheme: Option<String>
}

// `Box<>` is a container to hold some unknown type of objects. It allocates on
// the heap, so we don't want to use this often, but reading is a good place for
// it.
pub fn read_metadata(path: &str) -> Result<Metadata, Box<dyn std::error::Error>> {
  // `?` means to grab any error types and output them as the `Result`.
    let content: String = fs::read_to_string(path)?;

    let metadata: Metadata = parse_toml(&content)?;

    Ok(metadata)
}

pub fn write_metadata() {
  todo!("Working on it.")
}

// `dyn` is added by Rust analyzer/formatter, which is dynamically dispatched.
// The program can't determine the exact error type until runtime.
fn parse_toml(input: &str) -> Result<Metadata, dyn std::error::Error>{
  let metadata: Metadata = toml::from_str(input)
    .expect("Couldn't correctly parse the TOML string, is there a mistake in the structure?");
  Ok(metadata)
}

#[cfg(test)]
mod tests {
    // To import all code from above in this file.
    use super::*;

    #[test]
    fn test_fully_read() {
      let toml_str = r#"
title: "Random"
upload-type: "random"
creators:
  - name: "Jim"
    affiliation: "University"
    orcid: "12345"
related-identifiers:
  - identifier: "random"
    relation: "link"
    resource_type: "test"
      "#;

    }
}
