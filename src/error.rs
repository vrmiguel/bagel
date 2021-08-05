pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    FailedRequest(String),
    Reqwest(reqwest::Error),
    Deserialization(serde_json::Error),
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Deserialization(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}
