#![feature(cow_is_borrowed)]
#![deny(clippy::all)]
//! Turborepo's library for authenticating with the Vercel API.
//! Handles logging into Vercel, verifying SSO, and storing the token.

mod auth;
mod error;
mod server;
mod ui;

pub use auth::*;
pub use error::Error;
pub use server::*;

/// Token is the result of a successful login. It contains the token string and
/// a boolean indicating whether the token already existed on the filesystem.
pub struct Token {
    /// The actual token string.
    pub token: String,
    /// If this is `true`, it means this token already exists on the filesystem.
    /// If `false`, this is a new token.
    pub is_existing: bool,
}
