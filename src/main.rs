// The `main.rs` file is the binary entry-point, e.g. for CLIs

use clap::{Args, Parser, Subcommand};
use std::error::Error;
use std::path::{Path, PathBuf};
use zen_do::metadata::{Metadata, read_metadata, write_metadata};
use zen_do::sync::{CitationCff, ZenodoJson, sync_format};

// TODO: Include `verbose` flag everywhere with `clap-verbosity-flag`?
/// Common publishing tasks with Zenodo from the terminal.
#[derive(Parser)]
#[command(author, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a `.zenodo.toml` file with all deposit metadata fields.
    Init(InitArgs),

    /// List all deposits in your Zenodo account as raw JSON.
    List(ListArgs),

    /// Get the JSON of the deposit described by `.zenodo.toml`.
    Get(GetArgs),

    /// Sync `.zenodo.toml` into other formats (e.g. `CITATION.cff`).
    Sync(SyncArgs),

    /// Discards deposit draft changes.
    ///
    /// When running `update` or `publish` with  `--draft`, the deposit
    /// is left in an "editable" state. `discard` removes
    /// any changes and returns the deposit to its uneditable state.
    Discard(DiscardArgs),

    /// Publishes a Zenodo deposit as a record.
    ///
    /// Creates the record if it doesn't exist. To change only the metadata,
    /// use `update`.
    Publish(PublishArgs),

    /// Updates a deposit's or record's metadata from `.zenodo.toml`.
    ///
    /// Doesn't create a new DOI or change any files, only updates the
    /// metadata.
    Update(UpdateArgs),
}

#[derive(Args, Debug)]
struct SandboxArg {
    /// Whether to use the Zenodo sandbox environment (for testing).
    #[arg(long, short, default_value_t = false)]
    sandbox: bool,
}

#[derive(Args, Debug)]
struct MetadataPathArg {
    /// The path to `.zenodo.toml`.
    #[arg(default_value = ".zenodo.toml")]
    metadata_path: PathBuf,
}

#[derive(Args, Debug)]
struct DraftArg {
    /// Whether to create a draft Zenodo deposit, i.e., leave it in an
    /// "editable" state and not publish it.
    #[arg(long, short, default_value_t = false)]
    draft: bool,
}

#[derive(Args, Debug)]
pub struct InitArgs {
    /// The directory to initiate the `.zenodo.toml` in.
    #[arg(default_value = ".")]
    pub path: PathBuf,
}

#[derive(Args, Debug)]
struct ListArgs {
    #[command(flatten)]
    sandbox: SandboxArg,
}

#[derive(Args, Debug)]
struct UpdateArgs {
    #[command(flatten)]
    metadata_path: MetadataPathArg,

    #[command(flatten)]
    sandbox: SandboxArg,

    #[command(flatten)]
    draft: DraftArg,
}

#[derive(Args, Debug)]
struct DiscardArgs {
    #[command(flatten)]
    metadata_path: MetadataPathArg,

    #[command(flatten)]
    sandbox: SandboxArg,
}

#[derive(Args, Debug)]
struct GetArgs {
    #[command(flatten)]
    metadata_path: MetadataPathArg,

    #[command(flatten)]
    sandbox: SandboxArg,
}

#[derive(Args, Debug)]
struct PublishArgs {
    #[command(flatten)]
    metadata_path: MetadataPathArg,

    // TODO: Also as NonEmpty (or NonEmptyVec from non_empty_vec?)
    /// The path(s) to the file(s) to upload.
    #[arg(long, short, required = true)]
    files: Vec<PathBuf>,

    #[command(flatten)]
    sandbox: SandboxArg,

    #[command(flatten)]
    draft: DraftArg,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum ToFormats {
    ZenodoJson,
    CitationCff,
}

#[derive(Args, Debug)]
struct SyncArgs {
    #[command(flatten)]
    metadata_path: MetadataPathArg,

    /// The format to sync the metadata file to.
    #[arg(short, long, required = true)]
    to: ToFormats,
}

/// Create a `.zenodo.toml` file with all deposit metadata fields
///
/// # Arguments
///
/// - `args`: The arguments given to the function.
///
/// # Errors
///
/// Errors when writing to file, such as if there is a problem with the file
/// itself or where it will be saved.
pub fn init(args: &InitArgs) -> Result<(), Box<dyn Error>> {
    // makes any relative path explicit
    let path = std::fs::canonicalize(&args.path)?;

    let metadata_path = path.join(".zenodo.toml");

    if metadata_path.is_file() {
        return Err("A `.zenodo.toml` file already exists in this directory.".into());
    }

    // Rust cannot display a PathBuf, so we need to convert it to a str. A str
    // is restricted to utf-8.
    let project_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("Current directory contains unsupported characters")?;

    let metadata = Metadata::new(project_name);

    write_metadata(&metadata, &metadata_path)?;

    Ok(())
}

fn sync(args: &SyncArgs) -> Result<(), Box<dyn Error>> {
    let metadata_path = &args.metadata_path.metadata_path;
    let metadata = read_metadata(metadata_path)?;
    let parent = metadata_path.parent().unwrap_or(Path::new("."));

    match args.to {
        ToFormats::ZenodoJson => sync_format::<ZenodoJson>(&metadata, parent),
        ToFormats::CitationCff => sync_format::<CitationCff>(&metadata, parent),
    }
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Init(args) => match init(args) {
            Ok(()) => {
                print!("Created an empty `.zenodo.toml` file.");
            }
            Err(err) => {
                eprint!("Error: {err}");
            }
        },
        // TODO: Remove once implemented
        #[allow(unused_variables)]
        Commands::List(args) => todo!("Not started yet"),

        // TODO: Remove once implemented
        #[allow(unused_variables)]
        Commands::Get(args) => todo!("Not started yet"),

        Commands::Sync(args) => match sync(args) {
            Ok(()) => {
                print!("Synced metadata.");
            }
            Err(err) => {
                eprint!("Error: {err}");
            }
        },

        // TODO: Remove once implemented
        #[allow(unused_variables)]
        Commands::Discard(args) => todo!("Not started yet"),

        // TODO: Remove once implemented
        #[allow(unused_variables)]
        Commands::Publish(args) => todo!("Not started yet"),

        // TODO: Remove once implemented
        #[allow(unused_variables)]
        Commands::Update(args) => todo!("Not started yet"),
    }
}

#[cfg(test)]
mod tests {
    // To import all code from above in this file.
    use super::*;
    use ::zen_do::metadata::{
        Creator, RelatedIdentifier, RelatedIdentifierRelation, ZendoId, read_metadata,
    };
    use std::{collections::BTreeMap, fs};
    use zen_do::sync::{
        CffAuthor, CffIdentifier, CffIdentifierType, CitationCff, FormatFileName, JsonCreator,
        JsonRelatedIdentifier,
    };

    #[test]
    fn test_init_command_no_prior_file() {
        let temp_dir = tempfile::tempdir().unwrap();

        init(&InitArgs {
            path: temp_dir.path().to_path_buf(),
        })
        .unwrap();

        let metadata_path = temp_dir.path().join(".zenodo.toml");

        // 1. Was the file created?
        assert!(metadata_path.is_file());

        // 2. Should fail to read due to placeholder value:
        assert!(read_metadata(&metadata_path).is_err());

        // Remove placeholder:
        let contents = std::fs::read_to_string(&metadata_path).unwrap();
        let contents = contents
            .replace("<github-org>", "my-org")
            .replace("<output-type>", "docs");
        std::fs::write(&metadata_path, contents).unwrap();

        // 3. Can read_metadata parse it?
        let metadata = read_metadata(&metadata_path).unwrap();

        assert_eq!(metadata.title, "");
        assert_eq!(metadata.upload_type, "");
        assert_eq!(metadata.creators.len(), 1);
    }

    #[test]
    fn test_init_command_with_prior_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let metadata_path = temp_dir.path().join(".zenodo.toml");

        // Create an existing metadata file.
        std::fs::write(&metadata_path, "").unwrap();

        assert!(
            init(&InitArgs {
                path: temp_dir.path().to_path_buf(),
            })
            .is_err()
        );
    }

    #[test]
    fn sync_returns_error_when_metadata_file_missing() {
        let temp_dir = tempfile::tempdir().unwrap();
        let metadata_path = temp_dir.path().join(".zenodo.toml");

        assert!(
            sync(&SyncArgs {
                metadata_path: MetadataPathArg {
                    metadata_path: metadata_path.clone(),
                },
                to: ToFormats::ZenodoJson,
            })
            .is_err()
        );
    }

    #[test]
    fn sync_metadata_to_new_zenodo_json() {
        let temp_dir = tempfile::tempdir().unwrap();
        let (metadata_path, metadata) = create_zenodo_toml(temp_dir.path());

        sync(&SyncArgs {
            metadata_path: MetadataPathArg {
                metadata_path: metadata_path.clone(),
            },
            to: ToFormats::ZenodoJson,
        })
        .unwrap();

        let zenodo_json_path = metadata_path.parent().unwrap().join(ZenodoJson::FILE_NAME);
        let zenodo_json = read_zenodo_json(&zenodo_json_path);
        assert_zenodo_json_matches_metadata(&zenodo_json, &metadata);
        assert_eq!(zenodo_json.extra, BTreeMap::new());
    }

    #[test]
    fn sync_metadata_to_existing_zenodo_json_no_extra_fields() {
        let temp_dir = tempfile::tempdir().unwrap();
        let (zenodo_toml_path, metadata) = create_zenodo_toml(temp_dir.path());
        let (zenodo_json_path, old_zenodo_json) = create_zenodo_json(temp_dir.path(), None);

        sync(&SyncArgs {
            metadata_path: MetadataPathArg {
                metadata_path: zenodo_toml_path.clone(),
            },
            to: ToFormats::ZenodoJson,
        })
        .unwrap();

        let zenodo_json = read_zenodo_json(&zenodo_json_path);
        assert_zenodo_json_matches_metadata(&zenodo_json, &metadata);
        assert_eq!(zenodo_json.extra, old_zenodo_json.extra);
    }

    #[test]
    fn sync_metadata_to_existing_zenodo_json_with_extra_fields() {
        let temp_dir = tempfile::tempdir().unwrap();
        let (zenodo_toml_path, metadata) = create_zenodo_toml(temp_dir.path());
        let (zenodo_json_path, old_zenodo_json) = create_zenodo_json(
            temp_dir.path(),
            Some(BTreeMap::from([
                ("access_right".into(), "open".into()),
                (
                    "communities".into(),
                    serde_json::json!([
                        {
                            "identifier": "my-project"
                        }
                    ]),
                ),
            ])),
        );

        sync(&SyncArgs {
            metadata_path: MetadataPathArg {
                metadata_path: zenodo_toml_path.clone(),
            },
            to: ToFormats::ZenodoJson,
        })
        .unwrap();

        let zenodo_json = read_zenodo_json(&zenodo_json_path);
        assert_zenodo_json_matches_metadata(&zenodo_json, &metadata);
        assert_eq!(zenodo_json.extra, old_zenodo_json.extra);
    }

    #[test]
    fn sync_metadata_to_existing_zenodo_json_with_missing_fields() {
        let temp_dir = tempfile::tempdir().unwrap();
        let (zenodo_toml_path, metadata) = create_zenodo_toml(temp_dir.path());
        let zenodo_json_path = temp_dir.path().join(ZenodoJson::FILE_NAME);
        let old_zenodo_json = serde_json::json!({
        "creators": [
            {
              // Missing affiliation
                "name": "Old Test User",
                "orcid": "123"
            }
        ]
        });
        fs::write(
            &zenodo_json_path,
            serde_json::to_string(&old_zenodo_json).unwrap(),
        )
        .unwrap();

        sync(&SyncArgs {
            metadata_path: MetadataPathArg {
                metadata_path: zenodo_toml_path.clone(),
            },
            to: ToFormats::ZenodoJson,
        })
        .unwrap();

        let zenodo_json = read_zenodo_json(&zenodo_json_path);
        assert_zenodo_json_matches_metadata(&zenodo_json, &metadata);
        assert_eq!(zenodo_json.extra, BTreeMap::new());
    }

    #[test]
    fn sync_metadata_to_new_citation_cff() {
        let temp_dir = tempfile::tempdir().unwrap();
        let (metadata_path, metadata) = create_zenodo_toml(temp_dir.path());

        sync(&SyncArgs {
            metadata_path: MetadataPathArg {
                metadata_path: metadata_path.clone(),
            },
            to: ToFormats::CitationCff,
        })
        .unwrap();

        let citation_cff_path = metadata_path.parent().unwrap().join(CitationCff::FILE_NAME);
        let citation_cff = read_citation_cff(&citation_cff_path);

        assert_citation_cff_matches_metadata(&citation_cff, &metadata);
        assert_eq!(citation_cff.extra, BTreeMap::new());
    }

    #[test]
    fn sync_metadata_to_existing_citation_cff_no_extra_fields() {
        let temp_dir = tempfile::tempdir().unwrap();
        let (metadata_path, metadata) = create_zenodo_toml(temp_dir.path());
        let (citation_cff_path, old_citation_cff) = create_citation_cff(temp_dir.path(), None);

        sync(&SyncArgs {
            metadata_path: MetadataPathArg {
                metadata_path: metadata_path.clone(),
            },
            to: ToFormats::CitationCff,
        })
        .unwrap();

        let citation_cff = read_citation_cff(&citation_cff_path);

        assert_citation_cff_matches_metadata(&citation_cff, &metadata);
        assert_eq!(citation_cff.extra, old_citation_cff.extra);
    }

    #[test]
    fn sync_metadata_to_existing_citation_cff_with_extra_fields() {
        let temp_dir = tempfile::tempdir().unwrap();
        let (metadata_path, metadata) = create_zenodo_toml(temp_dir.path());
        let extra = BTreeMap::from([
            ("repository-code".into(), "https://example.com".into()),
            (
                "keywords".into(),
                serde_yaml::Value::Sequence(vec!["test".into()]),
            ),
        ]);
        let (citation_cff_path, old_citation_cff) =
            create_citation_cff(temp_dir.path(), Some(extra));

        sync(&SyncArgs {
            metadata_path: MetadataPathArg {
                metadata_path: metadata_path.clone(),
            },
            to: ToFormats::CitationCff,
        })
        .unwrap();

        let citation_cff = read_citation_cff(&citation_cff_path);

        assert_citation_cff_matches_metadata(&citation_cff, &metadata);
        assert_eq!(citation_cff.extra, old_citation_cff.extra);
    }

    #[test]
    fn sync_metadata_to_existing_citation_cff_with_missing_fields() {
        let temp_dir = tempfile::tempdir().unwrap();
        let (metadata_path, metadata) = create_zenodo_toml(temp_dir.path());
        let citation_cff_path = temp_dir.path().join(CitationCff::FILE_NAME);
        fs::write(
            &citation_cff_path,
            // Missing affiliation and ORCID
            r#"
authors:
  - given-names: Old
    family-names: User
"#,
        )
        .unwrap();

        sync(&SyncArgs {
            metadata_path: MetadataPathArg {
                metadata_path: metadata_path.clone(),
            },
            to: ToFormats::CitationCff,
        })
        .unwrap();

        let citation_cff = read_citation_cff(&citation_cff_path);

        assert_citation_cff_matches_metadata(&citation_cff, &metadata);
        assert_eq!(citation_cff.extra, BTreeMap::new());
    }

    fn create_zenodo_toml(temp_dir: &Path) -> (PathBuf, Metadata) {
        let metadata_path = temp_dir.join(".zenodo.toml");

        let metadata = Metadata {
            title: "Test Project".into(),
            upload_type: "book".into(),
            zendo_id: ZendoId::try_from("test:test-project:book".to_string()).unwrap(),
            creators: vec![Creator {
                name: "Test User".into(),
                affiliation: "Test Affiliation".into(),
                orcid: "0000-0000-0000-0000".into(),
            }],
            related_identifiers: vec![
                RelatedIdentifier {
                    identifier: "abcd123".into(),
                    relation: RelatedIdentifierRelation::IsIdenticalTo,
                    resource_type: "book".into(),
                    scheme: None,
                },
                RelatedIdentifier {
                    identifier: "10.1000/111".into(),
                    relation: RelatedIdentifierRelation::IsIdenticalTo,
                    resource_type: "book".into(),
                    scheme: Some("doi".into()),
                },
                RelatedIdentifier {
                    identifier: "https://example.com/project".into(),
                    relation: RelatedIdentifierRelation::IsIdenticalTo,
                    resource_type: "book".into(),
                    scheme: Some("url".into()),
                },
                RelatedIdentifier {
                    identifier: "swh:1:dir:0123456789abcdef0123456789abcdef01234567".into(),
                    relation: RelatedIdentifierRelation::IsIdenticalTo,
                    resource_type: "book".into(),
                    scheme: Some("swh".into()),
                },
                RelatedIdentifier {
                    identifier: "10.1000/112".into(),
                    relation: RelatedIdentifierRelation::IsDerivedFrom,
                    resource_type: "book".into(),
                    scheme: Some("doi".into()),
                },
            ],
        };
        write_metadata(&metadata, &metadata_path).unwrap();

        (metadata_path, metadata)
    }
    fn create_zenodo_json(
        temp_dir: &Path,
        extra: Option<BTreeMap<String, serde_json::Value>>,
    ) -> (PathBuf, ZenodoJson) {
        let zenodo_json_path = temp_dir.join(ZenodoJson::FILE_NAME);

        let zenodo_json = ZenodoJson {
            title: "Zenodo JSON Test Project".into(),
            upload_type: "poster".into(),
            creators: vec![JsonCreator {
                name: "Zenodo JSON User".into(),
                affiliation: "Zenodo JSON Affiliation".into(),
                orcid: "EFG".into(),
            }],
            related_identifiers: vec![JsonRelatedIdentifier {
                identifier: "efg789".into(),
                relation: RelatedIdentifierRelation::IsIdenticalTo,
                resource_type: "poster".into(),
                scheme: None,
            }],
            extra: extra.unwrap_or_default(),
        };
        fs::write(
            &zenodo_json_path,
            serde_json::to_string(&zenodo_json).unwrap(),
        )
        .unwrap();

        (zenodo_json_path, zenodo_json)
    }

    fn read_zenodo_json(path: &Path) -> ZenodoJson {
        assert!(path.is_file());
        let contents = std::fs::read_to_string(&path).unwrap();
        serde_json::from_str(&contents).unwrap()
    }

    fn assert_zenodo_json_matches_metadata(zenodo_json: &ZenodoJson, metadata: &Metadata) {
        assert_eq!(zenodo_json.title, metadata.title);
        assert_eq!(zenodo_json.upload_type, metadata.upload_type);
        assert_eq!(
            zenodo_json.creators,
            vec![JsonCreator {
                name: metadata.creators[0].name.clone(),
                affiliation: metadata.creators[0].affiliation.clone(),
                orcid: metadata.creators[0].orcid.clone(),
            }]
        );
        let mut expected_related_identifiers = metadata
            .related_identifiers
            .iter()
            .map(|related_identifier| JsonRelatedIdentifier {
                identifier: related_identifier.identifier.clone(),
                relation: related_identifier.relation.clone(),
                resource_type: related_identifier.resource_type.clone(),
                scheme: related_identifier.scheme.clone(),
            })
            .collect::<Vec<_>>();

        expected_related_identifiers.push(JsonRelatedIdentifier {
            identifier: metadata.zendo_id.to_urn(),
            relation: RelatedIdentifierRelation::IsIdenticalTo,
            resource_type: metadata.upload_type.clone(),
            scheme: Some("urn".into()),
        });
        assert_eq!(
            zenodo_json.related_identifiers,
            expected_related_identifiers
        );
    }

    fn create_citation_cff(
        temp_dir: &Path,
        extra: Option<BTreeMap<String, serde_yaml::Value>>,
    ) -> (PathBuf, CitationCff) {
        let citation_cff_path = temp_dir.join(CitationCff::FILE_NAME);

        let citation_cff = CitationCff {
            title: "CFF Test Project".into(),
            authors: vec![CffAuthor {
                given_names: "CFF".into(),
                family_names: "Test User".into(),
                affiliation: "CFF Test Affiliation".into(),
                orcid: "https://orcid.org/0000-0000-0000-0000".into(),
            }],
            identifiers: vec![CffIdentifier {
                value: "https://example.com".into(),
                identifier_type: CffIdentifierType::Url,
            }],
            extra: extra.unwrap_or_default(),
        };

        fs::write(
            &citation_cff_path,
            serde_yaml::to_string(&citation_cff).unwrap(),
        )
        .unwrap();

        (citation_cff_path, citation_cff)
    }

    fn read_citation_cff(path: &Path) -> CitationCff {
        assert!(path.is_file());
        let contents = fs::read_to_string(path).unwrap();
        serde_yaml::from_str(&contents).unwrap()
    }

    fn assert_citation_cff_matches_metadata(citation_cff: &CitationCff, metadata: &Metadata) {
        assert_eq!(citation_cff.title, metadata.title);

        assert_eq!(
            citation_cff.authors,
            vec![CffAuthor {
                given_names: metadata.creators[0].name.clone(),
                family_names: metadata.creators[0].name.clone(),
                affiliation: metadata.creators[0].affiliation.clone(),
                orcid: format!("https://orcid.org/{}", metadata.creators[0].orcid),
            }]
        );

        assert_eq!(
            citation_cff.identifiers,
            vec![
                CffIdentifier {
                    value: metadata.related_identifiers[0].identifier.clone(),
                    identifier_type: CffIdentifierType::Other,
                },
                CffIdentifier {
                    value: metadata.related_identifiers[1].identifier.clone(),
                    identifier_type: CffIdentifierType::Doi,
                },
                CffIdentifier {
                    value: metadata.related_identifiers[2].identifier.clone(),
                    identifier_type: CffIdentifierType::Url,
                },
                CffIdentifier {
                    value: metadata.related_identifiers[3].identifier.clone(),
                    identifier_type: CffIdentifierType::Swh,
                },
                CffIdentifier {
                    value: metadata.zendo_id.to_urn(),
                    identifier_type: CffIdentifierType::Other,
                },
            ]
        );
    }
}
