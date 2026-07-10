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

fn normalize_message(message: &str) -> String {
    message.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn format_entry(command: &Command, author: &str, timestamp: &Timestamp) -> Result<String> {
    let model = command.model.as_deref().unwrap_or("unspecified-model");
    let message = command.message()?;
    Ok(format!(
        "\n## {timestamp} — {model} — {author}\n\n{message}\n"
    ))
}

fn append_entry(command: &Command) -> Result<()> {
    let author = env::var("USER").unwrap_or_else(|_| "unknown".to_owned());
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
    let entry = format_entry(command, &author, &Timestamp::now())?;
    log.write_all(entry.as_bytes()).wrap_err_with(|| {
        format!(
            "Could not append to papercut log {}",
            command.file.display()
        )
    })?;
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let command = Command::parse();
    append_entry(&command)?;
    println!("Recorded papercut in {}.", command.file.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_model_file_and_message() {
        let result = Command::try_parse_from([
            "papercut",
            "-m",
            "gpt-5",
            "--file",
            "notes/PAPERCUTS.md",
            "broken",
            "link",
        ]);
        let command = match result {
            Ok(command) => command,
            Err(error) => panic!("command should parse: {error}"),
        };

        assert_eq!(command.model.as_deref(), Some("gpt-5"));
        assert_eq!(command.file, PathBuf::from("notes/PAPERCUTS.md"));
        let message = match command.message() {
            Ok(message) => message,
            Err(error) => panic!("message should normalize: {error}"),
        };
        assert_eq!(message, "broken link");
    }

    #[test]
    fn rejects_missing_message() {
        let result = Command::try_parse_from(["papercut", "-m", "gpt-5"]);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_an_all_whitespace_message() {
        let command = Command {
            model: None,
            file: PathBuf::from(DEFAULT_LOG_FILE),
            message: vec!["  \t\n ".to_owned()],
        };

        assert!(command.message().is_err());
    }

    #[test]
    fn formats_a_reviewable_markdown_entry() {
        let command = Command {
            model: Some("gpt-5".to_owned()),
            file: PathBuf::from(DEFAULT_LOG_FILE),
            message: vec!["The setup guide omits a required command.".to_owned()],
        };
        let timestamp = match Timestamp::from_second(1_783_665_546) {
            Ok(timestamp) => timestamp,
            Err(error) => panic!("timestamp should construct: {error}"),
        };

        let entry = match format_entry(&command, "ada", &timestamp) {
            Ok(entry) => entry,
            Err(error) => panic!("entry should format: {error}"),
        };
        assert_eq!(
            entry,
            "\n## 2026-07-10T06:39:06Z — gpt-5 — ada\n\nThe setup guide omits a required command.\n"
        );
    }
}
