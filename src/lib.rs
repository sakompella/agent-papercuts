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
struct Command {
    /// Model that encountered the friction.
    #[arg(short, long)]
    model: Option<String>,

    /// Markdown log to append to.
    #[arg(long, default_value = DEFAULT_LOG_FILE)]
    file: PathBuf,

    /// What happened and what would have prevented it.
    #[arg(required = true, num_args = 1..)]
    message: Vec<String>,
}

impl Command {
    fn message(&self) -> Result<String> {
        let message = self.message.join(" ");
        let message = normalize_message(&message);
        if message.is_empty() {
            bail!("A papercut message cannot be empty.");
        }

        Ok(message)
    }
}

/// Collapses every run of Unicode whitespace into one ASCII space.
#[must_use]
pub fn normalize_message(message: &str) -> String {
    helper::normalize_message(message)
}

/// Runs the command-line interface.
///
/// # Errors
///
/// Returns an error when the parsed message is invalid or its log cannot be
/// created or appended to.
pub fn run() -> Result<()> {
    let command = Command::parse();
    helper::append_entry(&command)?;
    println!("Recorded papercut in {}.", command.file.display());
    Ok(())
}
