use crate::metadata::Metadata;
use reqwest::blocking::{Client, Response};
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use serde::Deserialize;
use serde_json::Value;
use std::fmt::Write;
use std::time::Duration;

#[derive(Deserialize)]
pub struct Record {
    pub id: u64,
    pub state: State,
    pub submitted: bool,
    pub links: Links,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum State {
    Done,
    Inprogress,
    Unsubmitted,
    Error,
}

#[derive(Deserialize)]
pub struct Links {
    pub html: String,
    pub bucket: Option<String>,
}

pub struct ZenodoClient {
    client: Client,
    headers: HeaderMap,
    url: String,
}

impl ZenodoClient {
    pub fn new(token: &str, sandbox: bool) -> Result<Self, Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {token}"))?,
        );
        let host = if sandbox { "sandbox.zenodo" } else { "zenodo" };
        let client = Client::builder().timeout(Duration::from_secs(30)).build()?;

        Ok(Self {
            client,
            headers,
            url: format!("https://{host}.org/api/deposit/depositions"),
        })
    }
}

impl ZenodoClient {
    pub fn create(&self, metadata: &Metadata) -> Result<Record, Box<dyn std::error::Error>> {
        let response = self
            .client
            .post(&self.url)
            .headers(self.headers.clone())
            .json(&serde_json::json!({ "metadata": metadata }))
            .send()?;

        let response = raise_for_status_with_reason(response)?;
        Ok(response.json()?)
    }
}

fn raise_for_status_with_reason(
    response: Response,
) -> Result<Response, Box<dyn std::error::Error>> {
    let status = response.status();
    if status.is_success() {
        return Ok(response);
    }

    let body: Value = response.json().unwrap_or(Value::Null);
    let mut reason = body
        .get("message")
        .and_then(Value::as_str)
        .unwrap_or("N/A")
        .to_string();

    if let Some(errors) = body.get("errors").and_then(Value::as_array) {
        for error in errors {
            write!(reason, "\n- {error}")?;
        }
    }

    Err(format!("{status} Error: {reason}").into())
}
