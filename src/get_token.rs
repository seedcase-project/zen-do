use keyring::Entry;

const SERVICE_NAME: &str = "zen-do";

/// Gets the Zenodo token from the system keyring or environment variables.
///
/// # Arguments
///
/// - `sandbox`: Whether to get the token for the Zenodo sandbox environment or
///   the real API.
///
/// # Errors
///
/// Returns an error if neither the keyring nor the environment variable has a
/// usable (i.e., non-empty and unicode) Zenodo token.
pub fn get_token(sandbox: bool) -> Result<String, Box<dyn std::error::Error>> {
    let token_name = if sandbox {
        "ZENODO_SANDBOX_TOKEN"
    } else {
        "ZENODO_TOKEN"
    };

    match get_keyring_token(token_name) {
        Ok(token) => Ok(token),
        // If token not found in keyring, try environment variables.
        Err(keyring_error) => match get_env_token(token_name) {
            Ok(token) => Ok(token),
            // If token not found in env either.
            Err(env_error) => Err(format!(
                "No Zenodo access token found for '{token_name}' in the system keyring or the \
                     environment variables. You need to add it in order to access the Zenodo API.\n\
                     - Keyring: {keyring_error}\n\
                     - Environment variable: {env_error}"
            )
            .into()),
        },
    }
}

fn get_keyring_token(token_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let token = Entry::new(SERVICE_NAME, token_name)?.get_password()?;
    non_empty(token)
}

fn get_env_token(token_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let token = std::env::var(token_name)?;
    non_empty(token)
}

// Errors if the token is empty.
fn non_empty(token: String) -> Result<String, Box<dyn std::error::Error>> {
    if token.is_empty() {
        Err("The token is empty".into())
    } else {
        Ok(token)
    }
}
