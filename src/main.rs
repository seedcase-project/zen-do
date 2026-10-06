// The `main.rs` file is the binary entry-point, e.g. for CLIs

use clap::{Args, Parser, Subcommand};
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
    Init,

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
    /// Whether to create a draft Zenodo deposit, i.e., leave it in an "editable"
    /// state and not publish it.
    #[arg(long, short, default_value_t = false)]
    draft: bool,
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

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Init => todo!("Not started yet"),

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
