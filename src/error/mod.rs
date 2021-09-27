#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("http error with {0}")]
    HTTP(
        #[from]
        #[source]
        reqwest::Error,
    ),
}

#[derive(thiserror::Error, Debug)]
pub enum ResponseError {
    #[error("decode http response body with wrong format: {0}")]
    BodyFormat(
        #[from]
        #[source]
        reqwest::Error,
    ),
}
