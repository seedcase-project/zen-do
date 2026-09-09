// The `main.rs` file is the binary entry-point, e.g. for CLIs

use clap::{Args, Parser, Subcommand};

// TODO: Include `verbose` flag everywhere with `clap-verbosity-flag`?
/// Common publishing tasks with Zenodo from the command-line.
#[derive(Parser)]
#[command(author, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create an empty `.zenodo.toml` file that includes all the metadata
    /// fields for a deposit to be can be filled in.
    Init,

    /// List all Zenodo deposits in an account as raw JSON (direct from the
    /// Zenodo servers).
    List(ListArgs),

    /// Get the Zenodo deposit JSON based on the metadata file.
    Get(GetArgs),

    /// Converts the `.zenodo.toml` metadata file into other formats (e.g.
    /// `CITATION.cff`).
    Convert(ConvertArgs),

    /// Discards changes made to a Deposit (in the editable state). Whenever you
    /// use `--draft` in the other commands like `update` or `publish`, you
    /// change the Deposit into an "editable" state. Using `discard` removes
    /// any changes and changes the Deposit back to the uneditable state.
    Discard(DiscardArgs),

    /// Create or update a Zenodo deposit and then publish it as a record.
    /// Requires uploading a new file. If you only want to update the
    /// deposit's/record's metadata, use `update`.
    Publish(PublishArgs),

    /// Updates a Zenodo deposit's contents (and its record) with changes in the
    /// `.zenodo.toml`. This doesn't create a new DOI, only updates the
    /// metadata within the record (not any files).
    Update(UpdateArgs),
}

#[derive(Args, Debug)]
struct SandboxArg {
    /// Whether to use the Zenodo sandbox environment for testing purposes.
    #[arg(default_value_t = false)]
    sandbox: bool,
}

#[derive(Args, Debug)]
struct MetadataFileArg {
    /// The path to the metadata file.
    #[arg(default_value = ".zenodo.toml")]
    metadata_file: String,
}

#[derive(Args, Debug)]
struct DraftArg {
    /// Whether to create a draft Zenodo deposit, e.g. leave it in "editable"
    /// state and not publish it.
    #[arg(default_value_t = false)]
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

    /// The path to the file to upload.
    file: Option<Vec<String>>,

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
