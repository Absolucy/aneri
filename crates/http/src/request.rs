// SPDX-License-Identifier: MPL-2.0
use reqwest::{Method, header::HeaderMap};
use std::{
	error::Error,
	path::{Path, PathBuf},
	time::Duration,
};
use tokio::{fs::File, io::AsyncWriteExt};

const MAX_BODY_SIZE: usize = 16 * 1024 * 1024;

pub(crate) struct Request {
	pub method: String,
	pub url: String,
	pub headers: Vec<(String, String)>,
	pub body: Option<String>,
	pub body_file: Option<PathBuf>,
	pub output_file: Option<PathBuf>,
	pub timeout: Option<f32>,
}

pub(crate) struct Response {
	pub status: u16,
	pub headers: Vec<(String, String)>,
	pub body: Option<String>,
}

pub(crate) type Outcome = Result<Response, String>;

pub(crate) async fn run(request: Request) -> Outcome {
	let Some(secs) = request.timeout else {
		return exchange(request).await;
	};
	let limit =
		Duration::try_from_secs_f32(secs).map_err(|_| format!("invalid timeout: {secs}"))?;
	tokio::time::timeout(limit, exchange(request))
		.await
		.map_err(|_| format!("timed out after {secs} seconds"))?
}

async fn exchange(request: Request) -> Outcome {
	let method = Method::from_bytes(request.method.to_ascii_uppercase().as_bytes())
		.map_err(|_| format!("invalid method: {}", request.method))?;
	let mut builder = crate::client::get().request(method, &request.url);
	for (name, value) in &request.headers {
		builder = builder.header(name, value);
	}
	if let Some(path) = &request.body_file {
		let file = File::open(path)
			.await
			.map_err(|err| format!("failed to open {}: {err}", path.display()))?;
		builder = builder.body(file);
	} else if let Some(body) = request.body {
		builder = builder.body(body);
	}

	let mut response = builder.send().await.map_err(describe)?;
	let status = response.status().as_u16();
	let headers = collect_headers(response.headers());
	let body = match request.output_file.as_deref() {
		Some(path) => {
			write_body(&mut response, path).await?;
			None
		}
		None => Some(read_body(&mut response).await?),
	};
	Ok(Response {
		status,
		headers,
		body,
	})
}

async fn read_body(response: &mut reqwest::Response) -> Result<String, String> {
	let expected = response
		.content_length()
		.map_or(0, |len| len.min(MAX_BODY_SIZE as u64) as usize);
	let mut body = Vec::with_capacity(expected);
	while let Some(chunk) = response.chunk().await.map_err(describe)? {
		if body.len() + chunk.len() > MAX_BODY_SIZE {
			return Err("response body is over 16 MiB, use output_file to download it".to_owned());
		}
		body.extend_from_slice(&chunk);
	}
	Ok(String::from_utf8(body)
		.unwrap_or_else(|err| String::from_utf8_lossy(err.as_bytes()).into_owned()))
}

async fn write_body(response: &mut reqwest::Response, path: &Path) -> Result<(), String> {
	let fail = |err: std::io::Error| format!("failed to write {}: {err}", path.display());
	let mut file = File::create(path).await.map_err(fail)?;
	while let Some(chunk) = response.chunk().await.map_err(describe)? {
		file.write_all(&chunk).await.map_err(fail)?;
	}
	file.flush().await.map_err(fail)
}

fn collect_headers(headers: &HeaderMap) -> Vec<(String, String)> {
	headers
		.keys()
		.map(|name| {
			let value = headers
				.get_all(name)
				.iter()
				.map(|value| String::from_utf8_lossy(value.as_bytes()))
				.collect::<Vec<_>>()
				.join(", ");
			(name.as_str().to_owned(), value)
		})
		.collect()
}

fn describe(err: reqwest::Error) -> String {
	let mut text = err.to_string();
	let mut source = err.source();
	while let Some(cause) = source {
		text.push_str(": ");
		text.push_str(&cause.to_string());
		source = cause.source();
	}
	text
}
