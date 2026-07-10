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

pub fn normalize_message(message: &str) -> String {
    message.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn format_entry(command: &Command, author: &str, timestamp: &Timestamp) -> Result<String> {
    let model = command.model.as_deref().unwrap_or("unspecified-model");
    let message = command.message()?;
    Ok(format!(
        "\n## {timestamp} — {model} — {author}\n\n{message}\n"
    ))
}

pub fn append_entry(command: &Command) -> Result<()> {
    let author = env::var("USER").unwrap_or_else(|_| "unknown".to_owned());
    let entry = format_entry(command, &author, &Timestamp::now())?;
    let needs_heading = !command.file.exists();
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
    if needs_heading {
        log.write_all(INITIAL_LOG_CONTENT.as_bytes())
            .wrap_err_with(|| {
                format!(
                    "Could not initialize papercut log {}",
                    command.file.display()
                )
            })?;
    }
    log.write_all(entry.as_bytes()).wrap_err_with(|| {
        format!(
            "Could not append to papercut log {}",
            command.file.display()
        )
    })?;
    Ok(())
}
