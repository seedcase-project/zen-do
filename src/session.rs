use crate::deposit::Deposit;
use crate::metadata::ZendoId;
use reqwest::header;
use std::error::Error;

/// Zenodo session.
///
/// Holds everything needed to connect to Zenodo, send a request, and keep the
/// details from the response. Used with [`reqwest`](https://docs.rs/reqwest/latest/reqwest/)
/// to build and send requests to the Zenodo API.
///
/// Requests are synchronous because `zen-do` sends few, simple requests,
/// so it doesn't need any `async` structures.
pub struct Session {
    /// Headers sent with each request, including the authorization token.
    pub headers: header::HeaderMap,

    /// Zenodo depositions endpoint, for either production or the sandbox.
    pub url: reqwest::Url,

    /// Blocking client, reused for each request to Zenodo.
    pub connection: reqwest::blocking::Client,

    /// The deposit's Zendo ID. `None` if no ID was given.
    pub zendo_id: Option<ZendoId>,

    /// The deposit from Zenodo. `None` if it hasn't been fetched or
    /// wasn't found.
impl Session {
    /// Creates a new session for sending requests to Zenodo.
    ///
    /// Sets up the authorisation header, the depositions URL, and a reusable
    /// connection/client.
    ///
    /// # Arguments
    ///
    /// - `sandbox` (`bool`) - Whether to use the Zenodo sandbox or the real
    ///   Zenodo API.
    /// - `token` (`&str`) - The Zenodo personal access token.
    /// - `zendo_id` (`Option<ZendoId>`) - The Zendo ID of the deposit, if any.
    ///
    /// # Errors
    ///
    /// Errors if the token can't be used as a header value.
    pub fn new(
        sandbox: bool,
        token: &str,
        zendo_id: Option<ZendoId>,
    ) -> Result<Self, Box<dyn Error>> {
        let mut authorization_value = header::HeaderValue::from_str(&format!("Bearer {token}"))?;
        // Mark that the header represents sensitive information.
        authorization_value.set_sensitive(true);

        let mut headers = header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, authorization_value);
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let host = if sandbox { "sandbox.zenodo" } else { "zenodo" };
        let url = reqwest::Url::parse(&format!("https://{host}.org/api/deposit/depositions"))?;

        Ok(Session {
            headers,
            url,
            connection: reqwest::blocking::Client::new(),
            zendo_id,
            deposit: None, // TODO: Change when get_deposit() is implemented.
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_session_without_zendo_id() {
        let session = Session::new(false, "my_token", None).expect("valid session");

        assert!(session.headers[header::AUTHORIZATION].is_sensitive());
        assert_eq!(session.headers[header::AUTHORIZATION], "Bearer my_token");
        assert_eq!(
            session.url.as_str(),
            "https://zenodo.org/api/deposit/depositions"
        );
        assert_eq!(session.zendo_id, None);
    }

    #[test]
    fn test_new_sandbox_session_with_zendo_id() {
        let id_str = "name:project:type";
        let zendo_id = ZendoId::try_from(id_str.to_string()).expect("valid zendo_id");
        let sandbox_session =
            Session::new(true, "my_token", Some(zendo_id)).expect("valid session");

        assert!(sandbox_session.headers[header::AUTHORIZATION].is_sensitive());
        assert_eq!(
            sandbox_session.headers[header::AUTHORIZATION],
            "Bearer my_token"
        );
        assert_eq!(
            sandbox_session.url.as_str(),
            "https://sandbox.zenodo.org/api/deposit/depositions"
        );
        assert_eq!(
            sandbox_session.zendo_id.as_ref().map(ZendoId::as_str),
            Some(id_str)
        );
    }

    #[test]
    fn test_new_session_rejects_invalid_token() {
        assert!(Session::new(true, "bad\ntoken", None).is_err());
    }
}
