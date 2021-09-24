use std::{
    borrow::Cow,
    collections::HashMap,
    convert::TryFrom,
    fmt::{Debug, Display},
    sync::Arc,
};

use crate::{client::Configuration, credential::Credential, profile::Profile, region::Region};

pub use builder::RequestBuilder;
pub use iotcloud::*;
pub mod builder;
pub mod iotcloud;
pub mod scheme;

pub const ROOT_DOMAIN: &str = "tencentcloudapi.com";
pub const API_VERSION: &str = "2018-06-14";

pub trait ServiceRequest {
    fn service(&self) -> &'static str;
    fn action(&self) -> &'static str;
}

impl<T> From<RequestBuilder<T>> for reqwest::Request
where
    T: Flat + Debug + ServiceRequest + serde::Serialize,
{
    fn from(rb: RequestBuilder<T>) -> Self {
        let mut rb = rb.ensure().unwrap();
        let u = format!(
            "{}://{}{}",
            rb.scheme.as_ref(),
            rb.domain.as_ref().unwrap(),
            rb.path
        );

        dbg!(&rb);

        let mut request = reqwest::Request::new(rb.method.clone(), u.parse().unwrap());
        if let Some(payload) = rb.payload.take() {
            *request.body_mut() = Some(payload.into());
        }
        *request.headers_mut() = rb.headers;
        request
    }
}

pub trait IntoRequest {
    type Request: Default;

    fn into_request(self, config: Configuration) -> RequestBuilder<Self::Request>;
}

pub trait Flat {
    fn flat(&self) -> HashMap<String, String>;

    fn insert<T: ToString>(hm: &mut HashMap<String, String>, key: &str, value: &Option<T>) {
        if let Some(value) = value {
            hm.insert(key.to_string(), value.to_string());
        }
    }

    fn insert_slice<T: ToString + Display, V: AsRef<[T]>>(
        mut hm: &mut HashMap<String, String>,
        key: &str,
        values: &Option<V>,
    ) {
        if let Some(values) = values {
            for (index, value) in values.as_ref().iter().enumerate() {
                Self::insert(&mut hm, &format!("{}.{}", key, index), &Some(value));
            }
        }
    }
}
