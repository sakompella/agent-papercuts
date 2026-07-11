use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
};

use color_eyre::eyre::{Result, WrapErr};
use jiff::Timestamp;

use crate::Command;

const INITIAL_LOG_CONTENT: &str =
    "# Papercuts\n\nSmall, non-blocking workflow friction recorded by agents.\n";
const UNSPECIFIED_MODEL: &str = "unspecified-model";
const UNKNOWN_AUTHOR: &str = "unknown";

/// Collapses every run of Unicode whitespace into one ASCII space.
#[must_use]
pub fn normalize_message(message: &str) -> String {
    message.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn format_entry(command: &Command, author: &str, timestamp: &Timestamp) -> Result<String> {
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

pub fn append_entry(command: &Command) -> Result<()> {
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

fn heading_label(value: &str, fallback: &str) -> String {
    let label = normalize_message(value);
    if label.is_empty() {
        fallback.to_owned()
    } else {
        label
    }
}
