use std::{collections::HashMap, sync::Arc, time::Duration};

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Method,
};

use crate::{
    credential::Credential,
    profile::{ClientProfile, HTTProfile, Profile},
    region::Region,
    request::{BatchUpdateFirmwareRequest, RequestBuilder, Scheme, ServiceRequest},
};

pub mod iotcloud;

#[derive(Clone)]
pub struct Client {
    inner: reqwest::Client,
    region: Arc<Region>,
    profile: Arc<Profile>,
    credential: Arc<Credential>,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }
}

#[derive(Default)]
pub struct ClientBuilder {
    region: Option<Region>,
    cp: Option<ClientProfile>,
    hp: Option<HTTProfile>,
    credential: Option<Credential>,
}

impl ClientBuilder {
    pub fn region(mut self, region: Region) -> Self {
        self.region = Some(region);
        self
    }

    pub fn client_profile(mut self, cp: ClientProfile) -> Self {
        self.cp = Some(cp);
        self
    }

    pub fn http_profile(mut self, hp: HTTProfile) -> Self {
        self.hp = Some(hp);
        self
    }

    pub fn credential(mut self, credential: Credential) -> Self {
        self.credential = Some(credential);
        self
    }

    pub fn build(self) -> Option<Client> {
        // TODO: handle error
        let inner = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(self.hp.as_ref()?.timeout))
            .pool_idle_timeout(Duration::from_secs(3600))
            .pool_max_idle_per_host(100)
            .build()
            .unwrap();

        Some(Client {
            inner,
            region: Arc::new(self.region?),
            profile: Profile::new(self.cp?, self.hp?),
            credential: Arc::new(self.credential?),
        })
    }
}

pub struct ServiceClient<T> {
    client: Client,
    request: RequestBuilder<T>,
}

impl Client {
    pub fn iotcloud(&self) -> iotcloud::IOTClient {
        iotcloud::IOTClient::new(self.clone())
    }
}

impl<T> ServiceClient<T>
where
    T: Into<HashMap<String, String>> + ServiceRequest,
{
    pub fn send(self) -> Option<()> {
        let request = self.request.build()?;

        let mut headers = HeaderMap::new();
        let timestamp = request.params.get("Timestamp")?.to_string();
        let request_client = request.params.get("RequestClient")?.to_string();
        let language = request.profile.as_ref()?.client.language.clone();
        headers.insert("Host", request.domain.as_ref()?.parse().unwrap());
        headers.insert("X-TC-Action", request.action.as_ref()?.parse().unwrap());
        headers.insert("X-TC-Version", request.version.as_ref()?.parse().unwrap());
        headers.insert("X-TC-Timestamp", timestamp.parse().unwrap());
        headers.insert("X-TC-RequestClient", request_client.parse().unwrap());
        headers.insert("X-TC-Language", language.parse().unwrap());
        headers.insert(
            "X-TC-Region",
            self.client.region.to_string().parse().unwrap(),
        );
        if let Some(token) = self.client.credential.token() {
            headers.insert("X-TC-Token", token.parse().unwrap());
        }
        headers.insert("X-TC-Region", {
            let region = (*self.client.region).as_ref();
            region.parse().unwrap()
        });
        headers.insert(
            "Content-Type",
            match *(request.method.as_ref()?) {
                Method::GET => "application/x-www-form-urlencoded",
                _ => "application/json",
            }
            .parse()
            .unwrap(),
        );

        Some(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let client = Client::builder()
            .region(Region::APBeijing1)
            .build()
            .unwrap();
        client
            .iotcloud()
            .batch_update_firmware()
            .product_id("product_id")
            .send();
    }
}
