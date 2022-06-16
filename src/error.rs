/// Errors produced by this Library
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A Error that can occur when requesting web content
    #[error("An Error occurred while requesting web content: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Gallery {0} was not found")]
    NotFound(u32),
}
