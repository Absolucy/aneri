// SPDX-License-Identifier: MPL-2.0
use chrono::DateTime;
use meowtonin::byond_fn;
use std::{fs, path::Path};

const DEFAULT_DATE_FORMAT: &str = "%F";

/// The full hash of a git revision, like `HEAD` or `master~3`.
/// Returns null if there is no repository or the revision does not resolve.
#[byond_fn]
pub fn git_revparse(
	/// Anything `git rev-parse` accepts.
	rev: String,
) -> Option<String> {
	let repository = gix::discover(".").ok()?;
	let id = repository.rev_parse_single(rev.as_str()).ok()?;
	Some(id.to_string())
}

/// The commit date of a git revision, in UTC.
/// Returns null if there is no repository or the revision is not a commit.
/// Runtimes on an unknown `%` code.
#[byond_fn]
pub fn git_commit_date(
	/// Anything `git rev-parse` accepts.
	rev: String,
	/// A strftime-style string, like `%Y-%m-%d %H:%M`.
	/// Null gives `%F`, which is `YYYY-MM-DD`.
	format: Option<String>,
) -> Option<String> {
	let repository = gix::discover(".").ok()?;
	let id = repository.rev_parse_single(rev.as_str()).ok()?;
	let commit = id.object().ok()?.try_into_commit().ok()?;
	let seconds = commit.time().ok()?.seconds;
	format_seconds(seconds, format)
}

/// When `HEAD` last moved, in UTC.
/// That is the newest line of the `HEAD` reflog, so it is the date of the last checkout or commit, not of the commit itself.
/// Only reads `logs/HEAD`, so it works without the object database.
/// Returns null if there is no repository or the log is empty.
/// Runtimes on an unknown `%` code.
#[byond_fn]
pub fn git_commit_date_head(
	/// A strftime-style string, like `%Y-%m-%d %H:%M`.
	/// Null gives `%F`, which is `YYYY-MM-DD`.
	format: Option<String>,
) -> Option<String> {
	let (path, _) = gix::discover::upwards(Path::new(".")).ok()?;
	let (git_dir, _) = path.into_repository_and_work_tree_directories();
	let log = fs::read_to_string(git_dir.join("logs").join("HEAD")).ok()?;
	let line = log.lines().rev().find(|line| !line.trim().is_empty())?;
	// the tab splits the fixed part from the free-text message,
	// and the timestamp is second to last in the fixed part, after it comes the timezone.
	let (fixed, _) = line.split_once('\t').unwrap_or((line, ""));
	let seconds = fixed.split_ascii_whitespace().rev().nth(1)?.parse().ok()?;
	format_seconds(seconds, format)
}

fn format_seconds(seconds: i64, format: Option<String>) -> Option<String> {
	let format = format.as_deref().unwrap_or(DEFAULT_DATE_FORMAT);
	Some(
		DateTime::from_timestamp(seconds, 0)?
			.format(format)
			.to_string(),
	)
}
