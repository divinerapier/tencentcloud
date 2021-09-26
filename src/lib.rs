pub mod client;
pub mod credential;
pub mod error;
pub mod profile;
pub mod region;
pub mod request;

pub use credential::*;
pub use profile::*;
pub use region::Region;
pub use request::*;
pub use scheme::*;

pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
