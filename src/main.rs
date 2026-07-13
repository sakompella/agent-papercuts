use std::{
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

/// A parsed papercut log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Papercuts {
    pub entries: Vec<Papercut>,
}

/// One entry recorded in a [`Papercuts`] log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Papercut {
    pub timestamp: Timestamp,
    pub model: String,
    pub author: String,
    pub message: String,
}

impl Papercuts {
    /// Parses the canonical Markdown format produced by `papercut`.
    ///
    /// The log must have the standard preamble, and each entry must contain a
    /// UTC RFC 3339 timestamp, non-empty labels, and a normalized message.
    ///
    /// # Errors
    ///
    /// Returns an error when the document does not conform to the format
    /// produced by `papercut`.
    pub fn parse(contents: &str) -> Result<Self> {
        let entries = contents.strip_prefix(INITIAL_LOG_CONTENT).ok_or_else(|| {
            color_eyre::eyre::eyre!("Papercut log is missing its standard heading.")
        })?;

        if entries.is_empty() {
            return Ok(Self {
                entries: Vec::new(),
            });
        }

        let entries = entries.strip_prefix("\n## ").ok_or_else(|| {
            color_eyre::eyre::eyre!("Papercut entries must begin with a level-two heading.")
        })?;
        let entries = entries
            .strip_suffix('\n')
            .ok_or_else(|| color_eyre::eyre::eyre!("Papercut log must end with a newline."))?;

        let mut parts = entries.split("\n## ").peekable();
        let mut parsed_entries = Vec::new();
        let mut number = 1_usize;

        while let Some(entry) = parts.next() {
            let is_last = parts.peek().is_none();
            parsed_entries.push(Papercut::parse(entry, number, is_last)?);
            if !is_last {
                number = number
                    .checked_add(1)
                    .ok_or_else(|| color_eyre::eyre::eyre!("Too many papercut entries."))?;
            }
        }

        Ok(Self {
            entries: parsed_entries,
        })
    }
}

impl Papercut {
    fn parse(entry: &str, number: usize, is_last: bool) -> Result<Self> {
        let (heading, message) = entry.split_once("\n\n").ok_or_else(|| {
            color_eyre::eyre::eyre!("Papercut entry {number} is missing its message separator.")
        })?;
        let message = if is_last {
            message
        } else {
            message.strip_suffix('\n').ok_or_else(|| {
                color_eyre::eyre::eyre!("Papercut entry {number} must end with a newline.")
            })?
        };
        let mut labels = heading.split(" — ");
        let (Some(timestamp), Some(model), Some(author), None) =
            (labels.next(), labels.next(), labels.next(), labels.next())
        else {
            bail!("Papercut entry {number} must have timestamp, model, and author labels.");
        };

        if timestamp.is_empty()
            || model.is_empty()
            || author.is_empty()
            || normalize_message(model) != model
            || normalize_message(author) != author
        {
            bail!("Papercut entry {number} has an empty or unnormalized label.");
        }
        if message.is_empty() || normalize_message(message) != message {
            bail!("Papercut entry {number} has an empty or unnormalized message.");
        }

        let timestamp = timestamp
            .parse::<Timestamp>()
            .wrap_err_with(|| format!("Papercut entry {number} has an invalid timestamp."))?;

        Ok(Self {
            timestamp,
            model: model.to_owned(),
            author: author.to_owned(),
            message: message.to_owned(),
        })
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let command = CliOptions::parse();
    append_entry(&command)?;
    println!("Recorded papercut in {}.", command.file.display());
    Ok(())
}

#[derive(Debug, Parser)]
#[command(
    version,
    about,
    long_about = None,
    after_help = concat!("Project: ", env!("CARGO_PKG_REPOSITORY"))
)]
struct CliOptions {
    /// Model that encountered the friction.
    #[arg(short, long)]
    model: Option<String>,

    /// Person or agent that encountered the friction.
    #[arg(long)]
    author: Option<String>,

    /// Markdown log to append to.
    #[arg(long, default_value = DEFAULT_LOG_FILE)]
    file: PathBuf,

    /// What happened and what would have prevented it.
    #[arg(required = true, num_args = 1..)]
    message: Vec<String>,
}

impl CliOptions {
    fn message(&self) -> Result<String> {
        let message = normalize_message(&self.message.join(" "));
        if message.is_empty() {
            bail!("A papercut message cannot be empty.");
        }

        Ok(message)
    }
}

fn append_entry(options: &CliOptions) -> Result<()> {
    let author = options.author.as_deref().unwrap_or(UNKNOWN_AUTHOR);
    let entry = format_entry(options, author, &Timestamp::now())?;
    if let Some(parent) = options
        .file
        // returns Some("") if relative path root
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        // ensure file exists
        fs::create_dir_all(parent)
            .wrap_err_with(|| format!("Could not create log directory {}", parent.display()))?;
    }

    let mut log = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&options.file)
        .wrap_err_with(|| format!("Could not open papercut log {}", options.file.display()))?;

    let initial_contents = if log
        .metadata()
        .wrap_err_with(|| format!("Could not inspect papercut log {}", options.file.display()))?
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
            options.file.display()
        )
    })?;
    Ok(())
}

/// Formats one normalized papercut as a Markdown level-two log entry.
///
/// The model and author become single-line labels; the message must be non-empty.
fn format_entry(command: &CliOptions, author: &str, timestamp: &Timestamp) -> Result<String> {
    let model = heading_label(
        command.model.as_deref().unwrap_or(UNSPECIFIED_MODEL),
        UNSPECIFIED_MODEL,
    )?;
    let author = heading_label(author, UNKNOWN_AUTHOR)?;
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

fn heading_label(value: &str, fallback: &str) -> Result<String> {
    let label = normalize_message(value);
    let label = if label.is_empty() {
        fallback.to_owned()
    } else {
        label
    };

    if label.contains(" — ") {
        bail!("Papercut model and author labels cannot contain ` — `.");
    }

    Ok(label)
}
