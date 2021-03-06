use std::{fmt::Debug, sync::Arc, time::Duration};

use crate::{
    credential::Credential,
    profile::{ClientProfile, HTTProfile, Profile},
    region::Region,
    request::{RequestBuilder, ServiceRequest},
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

    pub fn build(mut self) -> crate::Result<Client> {
        if self.http_profile.root_domain.is_empty() {
            self.http_profile.root_domain = ROOT_DOMAIN.to_string();
        }
        // TODO: handle error
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(self.http_profile.timeout))
            .pool_idle_timeout(Duration::from_secs(3600))
            .pool_max_idle_per_host(100)
            .build()?;

        let config = Configuration {
            region: self.region,
            profile: Profile::new(self.client_profile, self.http_profile),
            credential: Arc::new(self.credential),
        };

        Ok(Client { client, config })
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
    pub async fn send<R: serde::de::DeserializeOwned>(
        self,
    ) -> crate::ResponseResult<crate::response::Response<R>> {
        let req: reqwest::Request = self.request.into();
        let client = self.client;
        let response = client.execute(req).await?;
        match response.json().await {
            Ok(r) => Ok(Ok(r)),
            Err(e) => Ok(Err(e.into())),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        BatchUpdateFirmwareRequest, BatchUpdateFirmwareResponse, ClientProfile, Credential,
        DescribeProductsRequest, DescribeProductsResponse, HTTProfile, Region,
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

        client
            .iotcloud()
            .batch_update_firmware(req)
            .send::<BatchUpdateFirmwareResponse>()
            .await
            .unwrap()
            .unwrap();
    }

    #[tokio::test]
    async fn test_describe_products() -> std::result::Result<(), Box<dyn std::error::Error>> {
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
            .build()?;

        let req = DescribeProductsRequest::builder()
            .set_offset(Some(0))
            .set_limit(Some(10));

        let resp = client
            .iotcloud()
            .describe_products(req)
            .send::<DescribeProductsResponse>()
            .await??;
        dbg!(resp);
        Ok(())
    }
}
