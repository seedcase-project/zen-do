// The `main.rs` file is the binary entry-point, e.g. for CLIs

use ::zen_do::metadata::{Metadata, write_metadata};
use clap::{Args, Parser, Subcommand};
use std::error::Error;
use std::path::PathBuf;

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

    /// Convert `.zenodo.toml` into other formats (e.g.
    /// `CITATION.cff`).
    Convert(ConvertArgs),

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
struct MetadataFileArg {
    /// The path to `.zenodo.toml`.
    #[arg(default_value = ".zenodo.toml")]
    metadata_file: PathBuf,
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
    metadata_file: MetadataFileArg,

    #[command(flatten)]
    sandbox: SandboxArg,

    #[command(flatten)]
    draft: DraftArg,
}

#[derive(Args, Debug)]
struct DiscardArgs {
    #[command(flatten)]
    metadata_file: MetadataFileArg,

    #[command(flatten)]
    sandbox: SandboxArg,
}

#[derive(Args, Debug)]
struct GetArgs {
    #[command(flatten)]
    metadata_file: MetadataFileArg,

    #[command(flatten)]
    sandbox: SandboxArg,
}

#[derive(Args, Debug)]
struct PublishArgs {
    #[command(flatten)]
    metadata_file: MetadataFileArg,

    // TODO: Also as NonEmpty (or NonEmptyVec from non_empty_vec?)
    /// The path(s) to the file(s) to upload.
    #[arg(long, short, required = true)]
    files: Vec<PathBuf>,

    #[command(flatten)]
    sandbox: SandboxArg,

    #[command(flatten)]
    draft: DraftArg,
}

#[derive(Args, Debug)]
struct ConvertArgs {
    #[command(flatten)]
    metadata_file: MetadataFileArg,

    // TODO: Convert `to` into an enum?
    /// The formats to convert the metadata file to. Can be a single format or
    /// an array of formats.
    #[arg(short, long, required = true)]
    to: Vec<String>,
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

        // TODO: Remove once implemented
        #[allow(unused_variables)]
        Commands::Convert(args) => todo!("Not started yet"),

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
    use ::zen_do::metadata::read_metadata;

    #[test]
    fn test_init_command_no_prior_file() {
        let temp_dir = tempfile::tempdir().unwrap();

        init(&InitArgs {
            path: temp_dir.path().to_path_buf(),
        })
        .unwrap();

        let metadata_file = temp_dir.path().join(".zenodo.toml");

        // 1. Was the file created?
        assert!(metadata_file.is_file());

        // 2. Should fail to read due to placeholder value:
        assert!(read_metadata(&metadata_file).is_err());

        // Remove placeholder:
        let contents = std::fs::read_to_string(&metadata_file).unwrap();
        let contents = contents
            .replace("<github-org>", "my-org")
            .replace("<output-type>", "docs");
        std::fs::write(&metadata_file, contents).unwrap();

        // 3. Can read_metadata parse it?
        let metadata = read_metadata(&metadata_file).unwrap();

        assert_eq!(metadata.title, "");
        assert_eq!(metadata.upload_type, "");
        assert_eq!(metadata.creators.len(), 1);
    }

    #[test]
    fn test_init_command_with_prior_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let metadata_file = temp_dir.path().join(".zenodo.toml");

        // Create an existing metadata file.
        std::fs::write(&metadata_file, "").unwrap();

        assert!(init(&InitArgs {
            path: temp_dir.path().to_path_buf(),
            })
        .is_err());
    }
}
