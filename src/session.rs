use crate::metadata::ZendoId;

/// Zenodo session.
///
/// Holds everything needed to connect to Zenodo, send a request, and keep the
/// details from the response. Used with [`reqwest`](https://docs.rs/reqwest/latest/reqwest/) to build and send requests
/// to the Zenodo API.
///
/// Requests are synchronous because `zen-do` sends few, simple requests,
/// so it doesn't need any `async` structures.
pub struct Session {
    /// Headers sent with each request, including the authorization token.
    pub headers: header::HeaderMap,

    /// Zenodo depositions endpoint, for either production or the sandbox.
    pub url: reqwest::Url, // TODO: Use URL type instead? Zenodo API uses string.

    /// Blocking client, reused for each request to Zenodo.
    pub connection: reqwest::blocking::Client,

    /// The deposit's Zendo ID. `None` if no ID was given.
    pub zendo_id: Option<ZendoId>,

    /// The deposit from Zenodo. `None` if it hasn't been fetched or
    /// wasn't found.
    pub deposit: Option<String>, // Option<Deposit> // TODO: Depends on #308
}
