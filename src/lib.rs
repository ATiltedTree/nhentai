pub(crate) mod api;
mod client;
mod error;
pub mod gallery;

pub type Result<T> = std::result::Result<T, error::Error>;

pub use client::Client;
pub use error::Error;
pub use gallery::Gallery;
