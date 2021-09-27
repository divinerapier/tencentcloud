pub mod client;
pub mod credential;
pub mod error;
pub mod profile;
pub mod region;
pub mod request;
pub mod response;

pub use credential::*;
pub use error::{Error, ResponseError};
pub use profile::*;
pub use region::Region;
pub use request::*;
pub use scheme::*;

pub type Result<T> = std::result::Result<T, Error>;

pub type ResponseResult<T> = Result<std::result::Result<T, ResponseError>>;
