use std::{fmt, future::Future, pin::Pin, sync::OnceLock, time::Duration};

use progenitor_client::Error as ProgenitorError;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

use crate::client::Client;

/// A token whose formatting always hides its contents.
#[derive(Clone)]
pub struct Secret(String);

impl Secret {
    /// Store a Hetzner Cloud API token.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Secret([REDACTED])")
    }
}

/// Retry configuration for blocking calls.
#[derive(Clone, Debug)]
pub struct RetryPolicy {
    /// Total number of attempts, including the first call.
    pub max_attempts: usize,
    /// Delay before the second attempt; later delays double up to `max_delay`.
    pub initial_delay: Duration,
    /// Maximum delay between attempts.
    pub max_delay: Duration,
}

/// HTTP method associated with a blocking generated call.
///
/// Retry is enabled only for methods whose requests are safe to repeat.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HttpMethod {
    Get,
    Head,
    Put,
    Delete,
    Options,
    Post,
    Patch,
    Other,
}

impl HttpMethod {
    fn is_retry_safe(self) -> bool {
        matches!(
            self,
            Self::Get | Self::Head | Self::Put | Self::Delete | Self::Options
        )
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 4,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(2),
        }
    }
}

impl RetryPolicy {
    fn delay_for(&self, retry_index: usize) -> Duration {
        self.initial_delay
            .checked_mul(2_u32.saturating_pow(retry_index.min(31) as u32))
            .unwrap_or(self.max_delay)
            .min(self.max_delay)
    }
}

/// Blocking facade over the generated async client.
///
/// Calls use a process-wide Tokio runtime shared by every instance. Do not
/// invoke blocking methods from inside an async Tokio task.
pub struct SyncClient {
    inner: Client,
    retry: RetryPolicy,
    secret: Secret,
}

/// Errors raised while configuring a blocking client.
#[derive(Debug, thiserror::Error)]
pub enum ClientBuildError {
    /// The token cannot be represented as an HTTP header.
    #[error("invalid API token header")]
    InvalidToken(#[from] reqwest::header::InvalidHeaderValue),
    /// The HTTP client could not be configured.
    #[error("HTTP client configuration failed: {0}")]
    Http(#[from] reqwest::Error),
}

impl fmt::Debug for SyncClient {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SyncClient")
            .field("base_url", &self.inner.baseurl)
            .field("retry", &self.retry)
            .field("token", &self.secret)
            .finish()
    }
}

impl SyncClient {
    /// Create a client for the standard Hetzner Cloud API endpoint.
    pub fn new(token: Secret) -> Result<Self, ClientBuildError> {
        Self::with_base_url("https://api.hetzner.cloud/v1", token)
    }

    /// Create a client with a custom API base URL.
    pub fn with_base_url(base_url: &str, token: Secret) -> Result<Self, ClientBuildError> {
        let mut headers = HeaderMap::new();
        let mut authorization = HeaderValue::from_str(&format!("Bearer {}", token.expose()))?;
        authorization.set_sensitive(true);
        headers.insert(AUTHORIZATION, authorization);
        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
        Ok(Self {
            inner: Client::new_with_client(base_url, http),
            retry: RetryPolicy::default(),
            secret: token,
        })
    }

    /// Set the retry policy used by [`Self::call`].
    pub fn with_retry_policy(mut self, retry: RetryPolicy) -> Self {
        self.retry = retry;
        self
    }

    /// Access the generated async client for direct async API calls.
    pub fn async_client(&self) -> &Client {
        &self.inner
    }

    /// Run one generated API call synchronously, retrying 429/5xx only when
    /// `method` is GET, HEAD, PUT, DELETE, or OPTIONS.
    ///
    /// The closure must build a fresh request on each invocation. This allows
    /// The method must match the generated operation passed in `operation`.
    /// At least one attempt is always made.
    pub fn call<T, E, F>(&self, method: HttpMethod, mut operation: F) -> Result<T, E>
    where
        T: 'static,
        E: RetryableError + 'static,
        F: for<'a> FnMut(&'a Client) -> Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'a>>,
    {
        let attempts = self.retry.max_attempts.max(1);
        runtime().block_on(async {
            for attempt in 0..attempts {
                match operation(&self.inner).await {
                    Ok(value) => return Ok(value),
                    Err(error)
                        if method.is_retry_safe()
                            && attempt + 1 < attempts
                            && error.is_retryable_status() =>
                    {
                        tokio::time::sleep(self.retry.delay_for(attempt)).await;
                    }
                    Err(error) => return Err(error),
                }
            }
            unreachable!("the retry loop always returns a result")
        })
    }
}

/// Classifies generated API errors for 429/5xx retries.
pub trait RetryableError {
    /// Whether the error came from a 429 or 5xx HTTP response.
    fn is_retryable_status(&self) -> bool;
}

impl<E> RetryableError for ProgenitorError<E> {
    fn is_retryable_status(&self) -> bool {
        retryable_status(self.status())
    }
}

fn retryable_status(status: Option<reqwest::StatusCode>) -> bool {
    status.is_some_and(|status| {
        status == reqwest::StatusCode::TOO_MANY_REQUESTS || status.is_server_error()
    })
}

fn runtime() -> &'static tokio::runtime::Runtime {
    static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("shared Hetzner runtime creation failed")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn generated_call_can_be_wrapped(client: &SyncClient) {
        let _ = client.call(HttpMethod::Get, |api| Box::pin(api.get_pricing()));
    }

    #[test]
    fn secret_debug_is_redacted() {
        let secret = Secret::new("fixture-token-do-not-print");
        assert_eq!(format!("{secret:?}"), "Secret([REDACTED])");
        let client = SyncClient::with_base_url("https://example.invalid/v1", secret).unwrap();
        assert!(!format!("{client:?}").contains("fixture-token-do-not-print"));
    }

    #[test]
    fn retry_delay_doubles_and_caps() {
        let retry = RetryPolicy {
            max_attempts: 4,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_millis(350),
        };
        assert_eq!(retry.delay_for(0), Duration::from_millis(100));
        assert_eq!(retry.delay_for(1), Duration::from_millis(200));
        assert_eq!(retry.delay_for(2), Duration::from_millis(350));
    }

    #[test]
    fn retries_only_rate_limits_and_server_errors() {
        assert!(retryable_status(Some(
            reqwest::StatusCode::TOO_MANY_REQUESTS
        )));
        assert!(retryable_status(Some(
            reqwest::StatusCode::INTERNAL_SERVER_ERROR
        )));
        assert!(!retryable_status(Some(reqwest::StatusCode::NOT_FOUND)));
        assert!(!retryable_status(None));
    }
}
