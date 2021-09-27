pub mod batch_update_firmware;
pub mod describe_products;

pub use batch_update_firmware::*;
pub use describe_products::*;

use super::{RequestBuilder, ServiceRequest, API_VERSION};
