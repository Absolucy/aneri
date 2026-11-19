// SPDX-License-Identifier: MPL-2.0
mod client;
mod request;

use aneri_core::{job, main_thread, runtime};
use meowtonin::{ByondResult, ByondValue, ToByond, byond_fn};
use request::{Outcome, Request};
use std::path::PathBuf;
use tokio::sync::Semaphore;

static IN_FLIGHT: Semaphore = Semaphore::const_new(64);

pub fn cleanup() {
	client::clear();
}

/// Starts an HTTP request in the background and returns its id.
/// At most 64 run at once, the rest wait their turn.
/// When it finishes, `/proc/__aneri_http_finish` is called with the id and the result.
#[byond_fn(name = "aneri_raw_http_send")]
pub fn http_send(
	/// `GET`, `POST`, and so on, in any case.
	method: String,
	url: String,
	/// Assoc list of header name to value.
	/// Values must be text.
	headers: Option<Vec<(String, String)>>,
	/// Text to send as the request body.
	body: Option<String>,
	/// Path of a file to upload as the body.
	/// Used instead of `body` if both are set.
	body_file: Option<PathBuf>,
	/// Path to save the response body to, for binary or large (over 16 MiB) responses.
	/// The returned body is null when this is set.
	output_file: Option<PathBuf>,
	/// Seconds the whole request may take, body included.
	/// Null waits forever.
	timeout: Option<f32>,
) -> String {
	let request = Request {
		method,
		url,
		headers: headers.unwrap_or_default(),
		body,
		body_file,
		output_file,
		timeout,
	};
	job::spawn(|id| async move {
		let _permit = IN_FLIGHT
			.acquire()
			.await
			.expect("http semaphore is never closed");
		let outcome = request::run(request).await;
		main_thread::queue(move || finish(id, outcome));
	})
	.to_string()
}

/// Starts an HTTP request in the background and never reports back.
/// The result, including any error, is thrown away.
#[byond_fn(name = "aneri_raw_http_send_and_forget")]
pub fn http_send_and_forget(
	/// `GET`, `POST`, and so on, in any case.
	method: String,
	url: String,
	/// Assoc list of header name to value.
	/// Values must be text.
	headers: Option<Vec<(String, String)>>,
	/// Text to send as the request body.
	body: Option<String>,
	/// Path of a file to upload as the body.
	/// Used instead of `body` if both are set.
	body_file: Option<PathBuf>,
	/// Path to save the response body to, for binary or large (over 16 MiB) responses.
	/// The returned body is null when this is set.
	output_file: Option<PathBuf>,
	/// Seconds the whole request may take, body included.
	/// Null waits forever.
	timeout: Option<f32>,
) {
	let request = Request {
		method,
		url,
		headers: headers.unwrap_or_default(),
		body,
		body_file,
		output_file,
		timeout,
	};
	runtime::handle().spawn(async move {
		let _permit = IN_FLIGHT
			.acquire()
			.await
			.expect("http semaphore is never closed");
		let _ = request::run(request).await;
	});
}

/// Runs an HTTP request and waits for it, freezing the server until it is done.
/// Returns `list(status_code, headers, body, error)`.
#[byond_fn(name = "aneri_raw_http_send_blocking")]
pub fn http_send_blocking(
	/// `GET`, `POST`, and so on, in any case.
	method: String,
	url: String,
	/// Assoc list of header name to value.
	/// Values must be text.
	headers: Option<Vec<(String, String)>>,
	/// Text to send as the request body.
	body: Option<String>,
	/// Path of a file to upload as the body.
	/// Used instead of `body` if both are set.
	body_file: Option<PathBuf>,
	/// Path to save the response body to, for binary or large (over 16 MiB) responses.
	/// The returned body is null when this is set.
	output_file: Option<PathBuf>,
	/// Seconds the whole request may take, body included.
	/// Null waits forever.
	timeout: Option<f32>,
) -> ByondResult<[ByondValue; 4]> {
	let request = Request {
		method,
		url,
		headers: headers.unwrap_or_default(),
		body,
		body_file,
		output_file,
		timeout,
	};
	outcome_values(runtime::handle().block_on(request::run(request)))
}

/// Cancels a request started with `aneri_raw_http_send`.
/// Its finish proc will not be called.
/// Returns false if the request already finished.
#[byond_fn(name = "aneri_raw_http_cancel")]
pub fn http_cancel(id: String) -> bool {
	id.parse().is_ok_and(job::cancel)
}

fn finish(id: u64, outcome: Outcome) {
	let Ok([status, headers, body, error]) = outcome_values(outcome) else {
		return;
	};
	let Ok(id) = id.to_string().to_byond() else {
		return;
	};
	let _ = meowtonin::call_global::<_, _, _, ByondValue>("__aneri_http_finish", [
		id, status, headers, body, error,
	]);
}

fn outcome_values(outcome: Outcome) -> ByondResult<[ByondValue; 4]> {
	match outcome {
		Ok(response) => Ok([
			response.status.to_byond()?,
			response.headers.to_byond()?,
			response.body.to_byond()?,
			ByondValue::NULL,
		]),
		Err(error) => Ok([
			ByondValue::NULL,
			ByondValue::NULL,
			ByondValue::NULL,
			error.to_byond()?,
		]),
	}
}
