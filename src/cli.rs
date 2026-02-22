use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Scan {
        #[arg(short, long)]
        input: String,
    },
    Batch {
        input_dir: String,
    },
}
