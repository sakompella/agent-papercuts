use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};

use hegel::generators;

#[expect(
    dead_code,
    reason = "this test imports main.rs only to property-test normalize_message"
)]
#[path = "../src/main.rs"]
mod papercut;

use papercut::normalize_message;

static NEXT_TEST_ID: AtomicU64 = AtomicU64::new(0);

struct TestLog {
    directory: PathBuf,
    path: PathBuf,
}

impl TestLog {
    fn new() -> Self {
        let id = NEXT_TEST_ID.fetch_add(1, Ordering::Relaxed);
        let directory =
            std::env::temp_dir().join(format!("agent-papercuts-test-{}-{id}", std::process::id()));
        if let Err(error) = fs::create_dir_all(&directory) {
            panic!("test directory should be created: {error}");
        }
        let path = directory.join("PAPERCUTS.md");

        Self { directory, path }
    }
}

impl Drop for TestLog {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.directory);
    }
}

fn run_papercut(arguments: &[&str]) -> Output {
    let output = Command::new(env!("CARGO_BIN_EXE_papercut"))
        .args(arguments)
        .output();
    match output {
        Ok(output) => output,
        Err(error) => panic!("papercut should run: {error}"),
    }
}

#[test]
fn appends_a_normalized_entry_to_the_requested_log() {
    let log = TestLog::new();
    let log_path = log.path.to_string_lossy();
    let output = run_papercut(&[
        "--file",
        &log_path,
        "--model",
        "gpt-5.6-terra",
        "broken",
        "  link",
    ]);

    assert!(output.status.success());
    let contents = match fs::read_to_string(&log.path) {
        Ok(contents) => contents,
        Err(error) => panic!("papercut log should be readable: {error}"),
    };
    assert!(contents.starts_with("# Papercuts\n\n"));
    assert!(contents.contains(" — gpt-5.6-terra — unknown\n\n"));
    assert!(contents.ends_with("broken link\n"));
}

#[test]
fn help_shows_project_information() {
    let output = run_papercut(&["--help"]);

    assert!(output.status.success());
    let help = String::from_utf8_lossy(&output.stdout);
    assert!(help.contains("vibed agent tool for logging simple issues"));
    assert!(help.contains("Project: https://github.com/sakompella/agent-papercuts"));
}

#[test]
fn rejects_a_missing_message() {
    let output = run_papercut(&["--model", "claude-sonnet-5"]);

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("required"));
}

#[test]
fn rejects_an_all_whitespace_message() {
    let log = TestLog::new();
    let log_path = log.path.to_string_lossy();
    let output = run_papercut(&["--file", &log_path, "  \t\n "]);

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("papercut message cannot be empty"));
    assert!(!log.path.exists());
}

#[test]
fn initializes_an_existing_empty_log() {
    let log = TestLog::new();
    if let Err(error) = fs::write(&log.path, "") {
        panic!("empty log should be created: {error}");
    }
    let log_path = log.path.to_string_lossy();
    let output = run_papercut(&["--file", &log_path, "A useful note."]);

    assert!(output.status.success());
    let contents = match fs::read_to_string(&log.path) {
        Ok(contents) => contents,
        Err(error) => panic!("papercut log should be readable: {error}"),
    };
    assert!(contents.starts_with("# Papercuts\n\n"));
}

#[test]
fn normalizes_model_and_author_labels() {
    let log = TestLog::new();
    let log_path = log.path.to_string_lossy();
    let output = run_papercut(&[
        "--file",
        &log_path,
        "--model",
        "gpt\n5.6",
        "--author",
        "test\nauthor",
        "A useful note.",
    ]);

    assert!(output.status.success());
    let contents = match fs::read_to_string(&log.path) {
        Ok(contents) => contents,
        Err(error) => panic!("papercut log should be readable: {error}"),
    };
    assert!(contents.contains(" — gpt 5.6 — test author\n\n"));
}

#[hegel::test]
fn normalizing_text_is_idempotent(tc: hegel::TestCase) {
    let text: String = tc.draw(generators::text());
    let normalized = normalize_message(&text);

    assert_eq!(normalize_message(&normalized), normalized);
}
