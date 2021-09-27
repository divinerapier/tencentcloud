use serde::Deserialize;
use std::collections::HashMap;

use serde::ser::SerializeStruct;

use crate::{client::Configuration, Flat, IntoRequest};

use super::{RequestBuilder, ServiceRequest, API_VERSION};

#[derive(Default, Debug)]
pub struct DescribeProductsRequest {
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct DescribeProductsResponse {
    #[serde(rename = "RequestId")]
    pub request_id: String,
    #[serde(rename = "TotalCount")]
    pub total_count: usize,
    #[serde(rename = "Products")]
    pub products: Vec<Product>,
}

#[derive(Deserialize, Debug)]
pub struct Product {
    #[serde(rename = "ProductId")]
    pub product_id: String,
    #[serde(rename = "ProductName")]
    pub product_name: String,
    #[serde(rename = "ProductMetadata")]
    pub product_metadata: ProductMetadata,
    #[serde(rename = "ProductProperties")]
    pub product_properties: ProductProperties,
}

#[derive(Deserialize, Debug)]
pub struct ProductMetadata {
    #[serde(rename = "CreationDate")]
    pub creation_date: i64,
}

#[derive(Deserialize, Debug)]
pub struct ProductProperties {
    #[serde(rename = "ProductDescription")]
    pub product_description: String,
    #[serde(rename = "EncryptionType")]
    pub encryption_type: String,
    #[serde(rename = "Region")]
    pub region: String,
    #[serde(rename = "ProductType")]
    pub product_type: i32,
    #[serde(rename = "Format")]
    pub format: String,
    #[serde(rename = "Platform")]
    pub platform: String,
    #[serde(rename = "Appeui")]
    pub appeui: String,
    #[serde(rename = "ModelId")]
    pub model_id: String,
    #[serde(rename = "ModelName")]
    pub model_name: String,
    #[serde(rename = "ProductKey")]
    pub product_key: String,
    #[serde(rename = "RegisterType")]
    pub register_type: i32,
    #[serde(rename = "ProductSecret")]
    pub product_secret: String,
    #[serde(rename = "RegisterLimit")]
    pub register_limit: i32,
}

impl DescribeProductsRequest {
    pub fn builder() -> DescribeProductsRequestBuilder {
        DescribeProductsRequestBuilder::default()
    }
}

impl IntoRequest for DescribeProductsRequestBuilder {
    type Request = DescribeProductsRequest;

    fn into_request(self, config: Configuration) -> RequestBuilder<Self::Request> {
        RequestBuilder {
            credential: config.credential.into(),
            profile: config.profile.into(),
            region: config.region,
            version: Some(API_VERSION.to_string()),
            inner: self.req,
            ..Default::default()
        }
    }
}

impl ServiceRequest for DescribeProductsRequest {
    fn service(&self) -> &'static str {
        "iotcloud"
    }

    fn action(&self) -> &'static str {
        "DescribeProducts"
    }
}

impl serde::Serialize for DescribeProductsRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("DescribeProductsRequest", 9)?;

        if let Some(ref offset) = self.offset {
            state.serialize_field("Offset", offset)?;
        }
        if let Some(ref limit) = self.limit {
            state.serialize_field("Limit", limit)?;
        }
        state.end()
    }
}

#[derive(Default)]
pub struct DescribeProductsRequestBuilder {
    req: DescribeProductsRequest,
}

impl DescribeProductsRequestBuilder {
    pub fn set_offset(mut self, offset: Option<u64>) -> Self {
        self.req.offset = offset;
        self
    }
    pub fn set_limit(mut self, limit: Option<u64>) -> Self {
        self.req.limit = limit;
        self
    }
}

impl Flat for DescribeProductsRequest {
    fn flat(&self) -> HashMap<String, String> {
        let mut hm = HashMap::new();
        Self::insert(&mut hm, "Offset", &self.offset);
        Self::insert(&mut hm, "Limit", &self.limit);
        hm
    }
}
