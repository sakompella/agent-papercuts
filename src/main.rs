use std::path::PathBuf;

use clap::Parser;
use color_eyre::eyre::{Result, bail};

mod helper;

const DEFAULT_LOG_FILE: &str = "PAPERCUTS.md";

#[derive(Debug, Parser)]
#[command(
    name = "papercut",
    about = "Record a small agent-workflow friction note in PAPERCUTS.md"
)]
pub(crate) struct Command {
    /// Model that encountered the friction.
    #[arg(short, long)]
    pub(crate) model: Option<String>,

    /// Markdown log to append to.
    #[arg(long, default_value = DEFAULT_LOG_FILE)]
    pub(crate) file: PathBuf,

    /// What happened and what would have prevented it.
    #[arg(required = true, num_args = 1..)]
    pub(crate) message: Vec<String>,
}

impl Command {
    pub(crate) fn message(&self) -> Result<String> {
        let message = self.message.join(" ");
        let message = helper::normalize_message(&message);
        if message.is_empty() {
            bail!("A papercut message cannot be empty.");
        }

        Ok(message)
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let command = Command::parse();
    helper::append_entry(&command)?;
    println!("Recorded papercut in {}.", command.file.display());
    Ok(())
}
