use std::path::PathBuf;

use clap::Parser;
use color_eyre::eyre::{Result, bail};
#[cfg(test)]
use jiff::Timestamp;

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

        let entry = match helper::format_entry(&command, "ada", &timestamp) {
            Ok(entry) => entry,
            Err(error) => panic!("entry should format: {error}"),
        };
        assert_eq!(
            entry,
            "\n## 2026-07-10T06:39:06Z — gpt-5 — ada\n\nThe setup guide omits a required command.\n"
        );
    }
}
