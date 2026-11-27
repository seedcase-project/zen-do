// The `main.rs` file is the binary entry-point, e.g. for CLIs

use clap::{Args, Parser, Subcommand};
use std::error::Error;
use std::path::{Path, PathBuf};
use zen_do::metadata::{read_metadata, write_init};
use zen_do::sync::{ZenodoJson, sync_format};

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

    write_init(&metadata_path)?;

    Ok(())
}

fn sync(args: &SyncArgs) -> Result<(), Box<dyn Error>> {
    let metadata_path = &args.metadata_path.metadata_path;
    let metadata = read_metadata(metadata_path)?;
    let parent = metadata_path.parent().unwrap_or(Path::new("."));

    match args.to {
        ToFormats::ZenodoJson => sync_format::<ZenodoJson>(&metadata, parent),
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
    use ::zen_do::metadata::{Creator, RelatedIdentifier, ZendoId, read_metadata, write_metadata, Metadata};
    use std::{collections::BTreeMap, fs};
    use zen_do::sync::{FormatFileName, JsonCreator, JsonRelatedIdentifier};

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
            .replace("<github-user>", "my-org")
            .replace("<repo-name>", "zen-do")
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
    fn sync_returns_error_when_metadata_path_missing() {
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

    fn create_zenodo_toml(temp_dir: &Path) -> (PathBuf, Metadata) {
        let metadata_path = temp_dir.join(".zenodo.toml");

        let metadata = Metadata {
            title: "Test Project".into(),
            upload_type: "book".into(),
            zendo_id: ZendoId::try_from("test:test-project:book".to_string()).unwrap(),
            creators: vec![Creator {
                name: "Test User".into(),
                affiliation: "Test Affiliation".into(),
                orcid: "ABC".into(),
            }],
            related_identifiers: vec![RelatedIdentifier {
                identifier: "abcd123".into(),
                relation: "isIdenticalTo".into(),
                resource_type: "book".into(),
                scheme: None,
            }],
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
                relation: "isIdenticalTo".into(),
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
        let expected_related_identifiers = vec![
            JsonRelatedIdentifier {
                identifier: metadata.related_identifiers[0].identifier.clone(),
                relation: metadata.related_identifiers[0].relation.clone(),
                resource_type: metadata.related_identifiers[0].resource_type.clone(),
                scheme: metadata.related_identifiers[0].scheme.clone(),
            },
            JsonRelatedIdentifier {
                identifier: format!("urn:zendo:{}", metadata.zendo_id.as_str()),
                relation: "isIdenticalTo".into(),
                resource_type: metadata.upload_type.clone(),
                scheme: Some("urn".into()),
            },
        ];
        assert_eq!(
            zenodo_json.related_identifiers,
            expected_related_identifiers
        );
    }
}
