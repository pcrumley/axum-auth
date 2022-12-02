//! High-level [http auth](https://developer.mozilla.org/en-US/docs/Web/HTTP/Authentication) extractors for [axum](https://github.com/tokio-rs/axum)
//!
//! # Usage
//!
//! Check out the following structures for more item-level documentation:
//!
//! - Basic auth: [AuthBasic]
//! - Bearer auth: [AuthBearer]
//!
//! That's all there is to it! Check our the [repository](https://github.com/owez/axum-auth) for contributing or some more documentation.

#[cfg(not(any(feature = "auth-basic", feature = "auth-bearer")))]
compile_error!(r#"At least one feature must be enabled!"#);

#[cfg(feature = "auth-basic")]
mod auth_basic;
#[cfg(feature = "auth-bearer")]
mod auth_bearer;

#[cfg(feature = "auth-basic")]
pub use auth_basic::AuthBasic;
#[cfg(feature = "auth-bearer")]
pub use auth_bearer::AuthBearer;

use http::{header::AUTHORIZATION, request::Parts, StatusCode};

/// Rejection error used in the [AuthBasicCustom] or [AuthBearerCustom] extractors
pub type Rejection = (StatusCode, &'static str);

/// The header is completely missing
pub(crate) const ERR_MISSING: &str = "`Authorization` header is missing";

/// The header has some invalid characters in it
pub(crate) const ERR_CHARS: &str = "`Authorization` header contains invalid characters";

/// The header couldn't be decoded properly for basic auth, might not have had a colon in the header
pub(crate) const ERR_DECODE: &str = "`Authorization` header could not be decoded";

/// The header was set as bearer authentication when we're expecting basic
pub(crate) const ERR_WRONG_BASIC: &str = "`Authorization` header must be for basic authentication";

/// The header was set as basic authentication when we're expecting bearer
pub(crate) const ERR_WRONG_BEARER: &str = "`Authorization` header must be a bearer token";

/// Helper trait for decoding [Parts] to a final extractor; this is the main interface into the decoding system
pub(crate) trait DecodeRequestParts: Sized {
    /// Decodes all provided [Parts] into a new instance of self, going through the entire decoding cycle
    ///
    /// To add custom errors here internally, set the `err_code` as something different
    fn decode_request_parts(req: &mut Parts, err_code: StatusCode) -> Result<Self, Rejection>;
}

/// Gets the auth header from [Parts] of the request or errors with [ERR_CHARS] or [ERR_MISSING] if wrong
pub(crate) fn get_header(parts: &mut Parts, err_code: StatusCode) -> Result<&str, Rejection> {
    parts
        .headers
        .get(AUTHORIZATION)
        .ok_or((err_code, ERR_MISSING))?
        .to_str()
        .map_err(|_| (err_code, ERR_CHARS))
}
