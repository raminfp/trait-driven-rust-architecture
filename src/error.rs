use thiserror::Error;

pub type Result<T> = core::result::Result<T, ErrorCode>;

#[derive(Debug, Error)]
pub enum ErrorCode {
    #[error("Invalid data format")] InvalidData,
    #[error("Missing required field: {0}")] MissingField(&'static str),
    #[error("Not owner of this resource")] NotOwner,
}
