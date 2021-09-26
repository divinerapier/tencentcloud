use core::panic;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    sync::Arc,
    time::Duration,
};

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Method,
};

use crate::{
    credential::Credential,
    profile::{ClientProfile, HTTProfile, Profile},
    region::Region,
    request::{BatchUpdateFirmwareRequest, RequestBuilder, ServiceRequest},
    Flat, IntoRequest, ROOT_DOMAIN,
};

pub mod iotcloud;

#[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
    config: Configuration,
}

#[derive(Clone)]
pub struct Configuration {
    pub region: Region,
    pub profile: Arc<Profile>,
    pub credential: Arc<Credential>,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }
}

#[derive(Default)]
pub struct ClientBuilder {
    region: Region,
    client_profile: ClientProfile,
    http_profile: HTTProfile,
    credential: Credential,
}

impl ClientBuilder {
    pub fn region(mut self, region: Region) -> Self {
        self.region = region;
        self
    }

    pub fn client_profile(mut self, client_profile: ClientProfile) -> Self {
        self.client_profile = client_profile;
        self
    }

    pub fn http_profile(mut self, http_profile: HTTProfile) -> Self {
        self.http_profile = http_profile;
        self
    }

    pub fn credential(mut self, credential: Credential) -> Self {
        self.credential = credential;
        self
    }

    pub fn build(mut self) -> Option<Client> {
        if self.http_profile.root_domain.is_empty() {
            self.http_profile.root_domain = ROOT_DOMAIN.to_string();
        }
        // TODO: handle error
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(self.http_profile.timeout))
            .pool_idle_timeout(Duration::from_secs(3600))
            .pool_max_idle_per_host(100)
            .build()
            .unwrap();

        let config = Configuration {
            region: self.region,
            profile: Profile::new(self.client_profile, self.http_profile),
            credential: Arc::new(self.credential),
        };

        Some(Client { client, config })
    }
}

pub struct ServiceClient<T> {
    client: reqwest::Client,
    request: RequestBuilder<T>,
}

impl<T> ServiceClient<T> {
    pub fn new<IR: IntoRequest<Request = T>>(client: Client, ir: IR) -> ServiceClient<T> {
        let request = ir.into_request(client.config);
        let client = client.client;
        ServiceClient { client, request }
    }
}

impl Client {
    pub fn iotcloud(&self) -> iotcloud::IOTClient {
        iotcloud::IOTClient::new(self.clone())
    }
}

impl<T> ServiceClient<T>
where
    T: Flat + ServiceRequest + Flat + Debug + serde::Serialize,
{
    pub async fn send(self) -> Option<()> {
        let req: reqwest::Request = self.request.into();
        let client = self.client;
        // TODO: Extract the response body and handle errors.
        // dbg!(&req);
        let response = client.execute(req).await.unwrap();
        println!("{:?}", response);
        let body = response.text().await.unwrap();
        println!("{:?}", body);
        Some(())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        BatchUpdateFirmwareRequest, ClientProfile, Credential, DescribeProductsRequest, HTTProfile,
        Region,
    };

    use super::Client;

    #[tokio::test]
    async fn test_batch_update_firmware() {
        let client = Client::builder()
            .region(Region::APBeijing1)
            .client_profile(ClientProfile::default())
            .http_profile(HTTProfile::default())
            .build()
            .unwrap();

        let req = BatchUpdateFirmwareRequest::builder().set_product_id("product_id".to_string());

        client.iotcloud().batch_update_firmware(req).send().await;
    }

    #[tokio::test]
    async fn test_describe_products() {
        let client = Client::builder()
            .region(Region::APBeijing)
            .client_profile(ClientProfile::default())
            .http_profile(HTTProfile::default())
            .credential(
                Credential::builder()
                    .access_key(std::env!("ACCESS_KEY"))
                    .secret_key(std::env!("SECRET_KEY"))
                    .build(),
            )
            .build()
            .unwrap();

        let req = DescribeProductsRequest::builder()
            .set_offset(Some(0))
            .set_limit(Some(10));

        client.iotcloud().describe_products(req).send().await;
    }
}
