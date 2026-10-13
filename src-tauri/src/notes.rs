//! Notes mode: transcriptions from the `transcribe_to_note` shortcut are
//! appended to a daily markdown inbox instead of being pasted.
//!
//! Capture is deliberately dumb and synchronous: every entry lands on disk
//! before anything else touches it, so nothing said is ever lost. Organizing
//! the inbox into topic notes happens later, as a separate step.

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use chrono::{Local, NaiveDateTime};
use tauri::{AppHandle, Manager};

use crate::settings::get_settings;

const INBOX_DIR: &str = "inbox";
const README_FILE: &str = "README.md";
const DEFAULT_DIR_NAME: &str = "Handy Notes";

const README: &str = "# Handy Notes

This folder is written by Handy's notes mode. Everything here is plain
markdown, so any editor or AI tool can read it.

- `inbox/YYYY-MM-DD.md`: raw voice captures for that day, one bullet per
  capture, prefixed with the local time (`- 14:05 ...`). Handy only ever
  appends to these files.
- Any other `.md` file is a note. You can create, edit, move or delete notes
  freely.
";

/// Where notes go when the user hasn't picked a folder.
pub fn default_notes_dir(app: &AppHandle) -> Result<PathBuf, String> {
    if let Ok(documents) = app.path().document_dir() {
        return Ok(documents.join(DEFAULT_DIR_NAME));
    }
    crate::portable::app_data_dir(app)
        .map(|dir| dir.join("notes"))
        .map_err(|e| format!("Failed to resolve notes directory: {}", e))
}

/// The notes folder currently in effect: the user's choice, else the default.
pub fn notes_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let settings = get_settings(app);
    match settings
        .notes_dir
        .as_deref()
        .map(str::trim)
        .filter(|dir| !dir.is_empty())
    {
        Some(dir) => Ok(PathBuf::from(dir)),
        None => default_notes_dir(app),
    }
}

/// Append a transcription to today's inbox. Returns the inbox file written.
pub fn capture(app: &AppHandle, text: &str) -> Result<PathBuf, String> {
    let dir = notes_dir(app)?;
    append_to_inbox(&dir, text, Local::now().naive_local())
        .map_err(|e| format!("Failed to save note in {}: {}", dir.display(), e))
}

fn append_to_inbox(dir: &Path, text: &str, now: NaiveDateTime) -> std::io::Result<PathBuf> {
    let inbox = dir.join(INBOX_DIR);
    fs::create_dir_all(&inbox)?;
    write_readme_if_missing(dir)?;

    let day = now.format("%Y-%m-%d").to_string();
    let path = inbox.join(format!("{}.md", day));
    let is_new = !path.exists();

    let mut entry = String::new();
    if is_new {
        entry.push_str(&format!("# Inbox {}\n\n", day));
    }
    entry.push_str(&format_entry(text, now));

    let mut file = OpenOptions::new().create(true).append(true).open(&path)?;
    file.write_all(entry.as_bytes())?;
    Ok(path)
}

fn write_readme_if_missing(dir: &Path) -> std::io::Result<()> {
    let path = dir.join(README_FILE);
    if path.exists() {
        return Ok(());
    }
    fs::write(path, README)
}

/// One bullet per capture. Line breaks are folded into spaces so a multi-line
/// transcription stays a single list item.
fn format_entry(text: &str, now: NaiveDateTime) -> String {
    let text = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    format!("- {} {}\n", now.format("%H:%M"), text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn at(hour: u32, minute: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2026, 10, 12)
            .unwrap()
            .and_hms_opt(hour, minute, 0)
            .unwrap()
    }

    #[test]
    fn first_capture_creates_dated_inbox_with_header() {
        let dir = tempfile::tempdir().unwrap();
        let path = append_to_inbox(dir.path(), "Buy drywall anchors", at(9, 5)).unwrap();

        assert_eq!(path, dir.path().join("inbox").join("2026-10-12.md"));
        assert_eq!(
            fs::read_to_string(&path).unwrap(),
            "# Inbox 2026-10-12\n\n- 09:05 Buy drywall anchors\n"
        );
    }

    #[test]
    fn later_captures_append_without_repeating_header() {
        let dir = tempfile::tempdir().unwrap();
        append_to_inbox(dir.path(), "First thought", at(9, 5)).unwrap();
        let path = append_to_inbox(dir.path(), "Second thought", at(14, 30)).unwrap();

        assert_eq!(
            fs::read_to_string(&path).unwrap(),
            "# Inbox 2026-10-12\n\n- 09:05 First thought\n- 14:30 Second thought\n"
        );
    }

    #[test]
    fn multi_line_transcription_stays_one_bullet() {
        assert_eq!(
            format_entry("  line one\n\n  line two  \n", at(21, 15)),
            "- 21:15 line one line two\n"
        );
    }

    #[test]
    fn readme_is_written_once_and_never_overwritten() {
        let dir = tempfile::tempdir().unwrap();
        append_to_inbox(dir.path(), "One", at(9, 0)).unwrap();
        let readme = dir.path().join(README_FILE);
        assert_eq!(fs::read_to_string(&readme).unwrap(), README);

        fs::write(&readme, "my own readme").unwrap();
        append_to_inbox(dir.path(), "Two", at(9, 1)).unwrap();
        assert_eq!(fs::read_to_string(&readme).unwrap(), "my own readme");
    }
}
