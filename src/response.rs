use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    #[serde(rename = "Response")]
    response: T,
}
