//! Generated client for the Hetzner Cloud API.
//!
//! The [`client`] module contains the async Progenitor client and generated
//! models. [`SyncClient`] provides blocking access using one shared Tokio
//! runtime and retries transient 429 and 5xx API responses.

mod sync;

/// Generated async client and API types.
#[allow(clippy::all)]
pub mod client;

pub use sync::{ClientBuildError, HttpMethod, RetryPolicy, RetryableError, Secret, SyncClient};
