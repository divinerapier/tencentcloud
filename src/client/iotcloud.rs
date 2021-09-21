use crate::request::{BatchUpdateFirmwareRequest, BatchUpdateFirmwareRequestBuilder};

use super::{Client, ServiceClient};

pub struct IOTClient {
    client: Client,
}

impl IOTClient {
    pub fn new(client: Client) -> IOTClient {
        IOTClient { client }
    }
}

impl IOTClient {
    pub fn batch_update_firmware(&self) -> BatchUpdateFirmwareClientBuilder {
        BatchUpdateFirmwareClientBuilder {
            client: self.client.clone(),
            request: BatchUpdateFirmwareRequest::builder(),
        }
    }
}

pub struct BatchUpdateFirmwareClientBuilder {
    client: Client,

    request: BatchUpdateFirmwareRequestBuilder,
}

impl BatchUpdateFirmwareClientBuilder {
    pub fn product_id(mut self, product_id: &str) -> Self {
        self.request = self.request.set_product_id(product_id.to_string());
        self
    }
    pub fn firmware_ori_version(mut self, firmware_ori_version: &str) -> Self {
        self.request = self
            .request
            .set_firmware_ori_version(firmware_ori_version.to_string());
        self
    }
    pub fn set_upgrade_method(mut self, upgrade_method: u64) -> Self {
        self.request = self.request.set_upgrade_method(upgrade_method);
        self
    }
    pub fn file_name(mut self, file_name: &str) -> Self {
        self.request = self.request.set_file_name(file_name.to_string());
        self
    }
    pub fn file_md5(mut self, file_md5: &str) -> Self {
        self.request = self.request.set_file_md5(file_md5.to_string());
        self
    }

    pub fn set_file_size(mut self, file_size: u64) -> Self {
        self.request = self.request.set_file_size(file_size);
        self
    }

    pub fn set_device_names(mut self, device_names: Vec<String>) -> Self {
        self.request = self.request.set_device_names(device_names);
        self
    }

    pub fn set_timeout_interval(mut self, timeout_interval: u64) -> Self {
        self.request = self.request.set_timeout_interval(timeout_interval);
        self
    }

    pub fn send(self) {
        let cred = self.client.credential.clone();
        let prof = self.client.profile.clone();
        let region = self.client.region.clone();
        let client = ServiceClient {
            request: self.request.into_request(cred, prof, region),
            client: self.client,
        };
        client.send();
    }
}
