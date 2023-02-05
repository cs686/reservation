use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReservationError {
    // #[error("data store disconnected")]
    // Disconnect(#[from] io::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader {
    //     expected: String,
    //     found: String,
    // },
    #[error("db error")]
    DbError(#[from] sqlx::Error),
    #[error("invalid time between start and end")]
    InvalidTime,
    #[error("unknown data store error")]
    Unknown,
}
