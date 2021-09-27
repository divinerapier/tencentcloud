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
