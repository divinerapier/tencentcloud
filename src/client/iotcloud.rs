use crate::{
    request::{BatchUpdateFirmwareRequest, BatchUpdateFirmwareRequestBuilder},
    DescribeProductsRequest, DescribeProductsRequestBuilder,
};

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
    pub fn batch_update_firmware(
        &self,
        req: BatchUpdateFirmwareRequestBuilder,
    ) -> ServiceClient<BatchUpdateFirmwareRequest> {
        ServiceClient::new(self.client.clone(), req)
    }

    pub fn describe_products(
        &self,
        req: DescribeProductsRequestBuilder,
    ) -> ServiceClient<DescribeProductsRequest> {
        ServiceClient::new(self.client.clone(), req)
    }
}

// pub struct BatchUpdateFirmwareClientBuilder {
//     pub request: BatchUpdateFirmwareRequestBuilder,
// }

// impl BatchUpdateFirmwareClientBuilder {
//     pub fn product_id(&mut self, product_id: &str) -> &Self {
//         self.request = self.request.set_product_id(product_id.to_string());
//         self
//     }
//     pub fn firmware_ori_version(mut self, firmware_ori_version: &str) -> Self {
//         self.request = self
//             .request
//             .set_firmware_ori_version(firmware_ori_version.to_string());
//         self
//     }
//     pub fn set_upgrade_method(mut self, upgrade_method: u64) -> Self {
//         self.request = self.request.set_upgrade_method(upgrade_method);
//         self
//     }
//     pub fn file_name(mut self, file_name: &str) -> Self {
//         self.request = self.request.set_file_name(file_name.to_string());
//         self
//     }
//     pub fn file_md5(mut self, file_md5: &str) -> Self {
//         self.request = self.request.set_file_md5(file_md5.to_string());
//         self
//     }

//     pub fn set_file_size(mut self, file_size: u64) -> Self {
//         self.request = self.request.set_file_size(file_size);
//         self
//     }

//     pub fn set_device_names(mut self, device_names: Vec<String>) -> Self {
//         self.request = self.request.set_device_names(device_names);
//         self
//     }

//     pub fn set_timeout_interval(mut self, timeout_interval: u64) -> Self {
//         self.request = self.request.set_timeout_interval(timeout_interval);
//         self
//     }

//     // pub async fn send(self) {
//     //     ServiceClient::new(self.client, self.request).send().await;
//     // }
// }
