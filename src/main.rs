use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

use clap::Parser;
use color_eyre::eyre::{Result, WrapErr, bail};
use jiff::Timestamp;

const DEFAULT_LOG_FILE: &str = "PAPERCUTS.md";
const INITIAL_LOG_CONTENT: &str =
    "# Papercuts\n\nSmall, non-blocking workflow friction recorded by agents.\n";
const UNSPECIFIED_MODEL: &str = "unspecified-model";
const UNKNOWN_AUTHOR: &str = "unknown";

fn main() -> Result<()> {
    color_eyre::install()?;
    let command = Command::parse();
    append_entry(&command)?;
    println!("Recorded papercut in {}.", command.file.display());
    Ok(())
}

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
        let message = normalize_message(&self.message.join(" "));
        if message.is_empty() {
            bail!("A papercut message cannot be empty.");
        }

        Ok(message)
    }
}

fn append_entry(command: &Command) -> Result<()> {
    let author = env::var("USER").unwrap_or_else(|_| UNKNOWN_AUTHOR.to_owned());
    let entry = format_entry(command, &author, &Timestamp::now())?;
    if let Some(parent) = command
        .file
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)
            .wrap_err_with(|| format!("Could not create log directory {}", parent.display()))?;
    }

    let mut log = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&command.file)
        .wrap_err_with(|| format!("Could not open papercut log {}", command.file.display()))?;
    let initial_contents = if log
        .metadata()
        .wrap_err_with(|| format!("Could not inspect papercut log {}", command.file.display()))?
        .len()
        == 0
    {
        INITIAL_LOG_CONTENT
    } else {
        ""
    };
    let payload = format!("{initial_contents}{entry}");
    log.write_all(payload.as_bytes()).wrap_err_with(|| {
        format!(
            "Could not append to papercut log {}",
            command.file.display()
        )
    })?;
    Ok(())
}

/// Formats one normalized papercut as a Markdown level-two log entry.
///
/// The model and author become single-line labels; the message must be non-empty.
fn format_entry(command: &Command, author: &str, timestamp: &Timestamp) -> Result<String> {
    let model = heading_label(
        command.model.as_deref().unwrap_or(UNSPECIFIED_MODEL),
        UNSPECIFIED_MODEL,
    );
    let author = heading_label(author, UNKNOWN_AUTHOR);
    let message = command.message()?;
    Ok(format!(
        "\n## {timestamp} — {model} — {author}\n\n{message}\n"
    ))
}

/// Collapses every run of Unicode whitespace into one ASCII space.
#[must_use]
pub fn normalize_message(message: &str) -> String {
    message.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn heading_label(value: &str, fallback: &str) -> String {
    let label = normalize_message(value);
    if label.is_empty() {
        fallback.to_owned()
    } else {
        label
    }
}
