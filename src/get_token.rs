use keyring::Entry;

const SERVICE_NAME: &str = "zen-do";

/// Gets the Zenodo token from the system keyring or environment variables.
///
/// Tokens must be named either `ZENODO_TOKEN` or `ZENODO_SANDBOX_TOKEN`.
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

#[cfg(test)]
mod tests {
    use super::*;
    use ::keyring_core::mock;

    const SANDBOX_TOKEN_NAME: &str = "ZENODO_SANDBOX_TOKEN";
    const TOKEN_NAME: &str = "ZENODO_TOKEN";

    fn use_empty_mock_keyring() {
        // Initialise the real store first, so it doesn't replace the mock when
        // get_token() is called.
        let _ = Entry::store_status();
        keyring_core::set_default_store(mock::Store::new().expect("a mock store is created"));
    }

    fn use_failing_keyring() {
        // Initialise the real store first, so it isn't set up again in get_token()
        // after being removed.
        let _ = Entry::store_status();
        keyring_core::unset_default_store();
    }

    fn set_keyring_token(token_name: &str, token: &str) {
        let entry = Entry::new(SERVICE_NAME, token_name).expect("a mock entry is created");
        entry.set_password(token).expect("password is set");
    }

    #[test]
    fn gets_keyring_token_over_env_token() {
        temp_env::with_var(SANDBOX_TOKEN_NAME, Some("env_token"), || {
            use_empty_mock_keyring();
            set_keyring_token(SANDBOX_TOKEN_NAME, "keyring_token");

            let token = get_token(true).expect("a token should be found");

            assert_eq!(token, "keyring_token")
        });
    }

    #[test]
    fn gets_env_token_when_no_token_in_keyring() {
        temp_env::with_var(SANDBOX_TOKEN_NAME, Some("env_token"), || {
            use_empty_mock_keyring();

            let token = get_token(true).expect("a token should be found");

            assert_eq!(token, "env_token")
        });
    }

    #[test]
    fn gets_env_token_when_empty_token_in_keyring() {
        temp_env::with_var(SANDBOX_TOKEN_NAME, Some("env_token"), || {
            use_empty_mock_keyring();
            set_keyring_token(SANDBOX_TOKEN_NAME, "");

            let token = get_token(true).expect("a token should be found");

            assert_eq!(token, "env_token")
        });
    }

    #[test]
    fn gets_env_token_when_keyring_fails() {
        temp_env::with_var(SANDBOX_TOKEN_NAME, Some("env_token"), || {
            use_failing_keyring();

            let token = get_token(true).expect("a token should be found");

            assert_eq!(token, "env_token")
        });
    }

    #[test]
    fn gets_keyring_token_when_sandbox_is_false() {
        temp_env::with_vars(
            [
                (TOKEN_NAME, Some("env_token")),
                (SANDBOX_TOKEN_NAME, Some("sandbox_token")),
            ],
            || {
                use_empty_mock_keyring();
                set_keyring_token(TOKEN_NAME, "keyring_token");

                let token = get_token(false).expect("a token should be found");

                assert_eq!(token, "keyring_token")
            },
        )
    }

    #[test]
    fn errors_when_empty_env_token_is_found() {
        temp_env::with_var(SANDBOX_TOKEN_NAME, Some(""), || {
            // No keyring token is set.
            use_empty_mock_keyring();

            let token_error = get_token(true).expect_err("an empty token should not be accepted");

            assert!(token_error.to_string().contains("empty"))
        });
    }

    #[test]
    fn errors_when_no_tokens_are_found() {
        temp_env::with_var_unset(SANDBOX_TOKEN_NAME, || {
            use_empty_mock_keyring();

            let token_error = get_token(true).expect_err("no token should be found");

            assert!(token_error.to_string().contains(SANDBOX_TOKEN_NAME))
        });
    }

    #[test]
    fn errors_with_failing_keyring() {
        temp_env::with_var_unset(SANDBOX_TOKEN_NAME, || {
            use_failing_keyring();

            let error_message = get_token(true)
                .expect_err("no token should be found")
                .to_string();

            assert!(error_message.contains(&keyring::Error::NoDefaultStore.to_string()))
        });
    }
}
