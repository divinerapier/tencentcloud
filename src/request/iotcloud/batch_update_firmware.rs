use serde::Deserialize;
use std::collections::HashMap;

use serde::ser::SerializeStruct;

use crate::{client::Configuration, Flat, IntoRequest};

use super::{RequestBuilder, ServiceRequest, API_VERSION};

#[derive(Default, Debug)]
pub struct BatchUpdateFirmwareRequest {
    // #[serde(rename = "ProductID")]
    pub product_id: Option<String>,
    // #[serde(rename = "FirmwareVersion")]
    pub firmware_version: Option<String>,
    // #[serde(rename = "FirmwareOriVersion")]
    pub firmware_ori_version: Option<String>,
    // #[serde(rename = "UpgradeMethod")]
    pub upgrade_method: Option<u64>,
    // #[serde(rename = "FileName")]
    pub file_name: Option<String>,
    // #[serde(rename = "FileMd5")]
    pub file_md5: Option<String>,
    // #[serde(rename = "FileSize")]
    pub file_size: Option<u64>,
    // #[serde(rename = "DeviceNames")]
    pub device_names: Option<Vec<String>>,
    // #[serde(rename = "TimeoutInterval")]
    pub timeout_interval: Option<u64>,
}

#[derive(Deserialize)]
pub struct BatchUpdateFirmwareResponse {}

impl serde::Serialize for BatchUpdateFirmwareRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("BatchUpdateFirmwareRequest", 9)?;

        if let Some(ref product_id) = self.product_id {
            state.serialize_field("ProductID", product_id)?;
        }
        if let Some(ref firmware_version) = self.firmware_version {
            state.serialize_field("FirmwareVersion", firmware_version)?;
        }
        if let Some(ref firmware_ori_version) = self.firmware_ori_version {
            state.serialize_field("FirmwareOriVersion", firmware_ori_version)?;
        }
        if let Some(ref upgrade_method) = self.upgrade_method {
            state.serialize_field("UpgradeMethod", upgrade_method)?;
        }
        if let Some(ref file_name) = self.file_name {
            state.serialize_field("FileName", file_name)?;
        }
        if let Some(ref file_md5) = self.file_md5 {
            state.serialize_field("FileMd5", file_md5)?;
        }
        if let Some(ref file_size) = self.file_size {
            state.serialize_field("FileSize", file_size)?;
        }
        if let Some(ref device_names) = self.device_names {
            if !device_names.is_empty() {
                state.serialize_field("DeviceNames", device_names)?;
            }
        }
        if let Some(ref timeout_interval) = self.timeout_interval {
            state.serialize_field("TimeoutInterval", timeout_interval)?;
        }
        state.end()
    }
}

impl ServiceRequest for BatchUpdateFirmwareRequest {
    fn service(&self) -> &'static str {
        "iotcloud"
    }

    fn action(&self) -> &'static str {
        "BatchUpdateFirmware"
    }
}

#[derive(Default)]
pub struct BatchUpdateFirmwareRequestBuilder {
    request: BatchUpdateFirmwareRequest,
}

impl BatchUpdateFirmwareRequestBuilder {
    pub fn set_product_id(mut self, product_id: String) -> Self {
        self.request.product_id = Some(product_id);
        self
    }
    pub fn set_firmware_version(mut self, firmware_version: String) -> Self {
        self.request.firmware_version = Some(firmware_version);
        self
    }
    pub fn set_firmware_ori_version(mut self, firmware_ori_version: String) -> Self {
        self.request.firmware_ori_version = Some(firmware_ori_version);
        self
    }
    pub fn set_upgrade_method(mut self, upgrade_method: u64) -> Self {
        self.request.upgrade_method = Some(upgrade_method);
        self
    }
    pub fn set_file_name(mut self, file_name: String) -> Self {
        self.request.file_name = Some(file_name);
        self
    }
    pub fn set_file_md5(mut self, file_md5: String) -> Self {
        self.request.file_md5 = Some(file_md5);
        self
    }
    pub fn set_file_size(mut self, file_size: u64) -> Self {
        self.request.file_size = Some(file_size);
        self
    }
    pub fn set_device_names(mut self, device_names: Vec<String>) -> Self {
        self.request.device_names = Some(device_names);
        self
    }
    pub fn set_timeout_interval(mut self, timeout_interval: u64) -> Self {
        self.request.timeout_interval = Some(timeout_interval);
        self
    }
}

impl IntoRequest for BatchUpdateFirmwareRequestBuilder {
    type Request = BatchUpdateFirmwareRequest;

    fn into_request(self, config: Configuration) -> RequestBuilder<Self::Request> {
        RequestBuilder {
            credential: config.credential.into(),
            profile: config.profile.into(),
            region: config.region,
            version: Some(API_VERSION.to_string()),
            inner: self.request,
            ..Default::default()
        }
    }
}

impl BatchUpdateFirmwareRequest {
    pub fn builder() -> BatchUpdateFirmwareRequestBuilder {
        BatchUpdateFirmwareRequestBuilder {
            request: Default::default(),
        }
    }
}

impl Flat for BatchUpdateFirmwareRequest {
    fn flat(&self) -> HashMap<String, String> {
        let mut hm = HashMap::new();
        Self::insert(&mut hm, "ProductID", &self.product_id);
        Self::insert(&mut hm, "FirmwareVersion", &self.firmware_version);
        Self::insert(&mut hm, "FirmwareOriVersion", &self.firmware_ori_version);
        Self::insert(&mut hm, "UpgradeMethod", &self.upgrade_method);
        Self::insert(&mut hm, "FileName", &self.file_name);
        Self::insert(&mut hm, "FileMd5", &self.file_md5);
        Self::insert(&mut hm, "FileSize", &self.file_size);
        Self::insert(&mut hm, "TimeoutInterval", &self.timeout_interval);
        Self::insert_slice(&mut hm, "DeviceNames", &self.device_names);
        hm
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let req = super::BatchUpdateFirmwareRequest {
            product_id: None,
            firmware_version: None,
            firmware_ori_version: None,
            upgrade_method: None,
            file_name: None,
            file_md5: None,
            file_size: None,
            device_names: None,
            timeout_interval: None,
        };
        println!("{}", serde_json::to_string(&req).unwrap());

        let req = super::BatchUpdateFirmwareRequest {
            product_id: None,
            firmware_version: None,
            firmware_ori_version: None,
            upgrade_method: None,
            file_name: None,
            file_md5: None,
            file_size: None,
            device_names: Some(vec![]),
            timeout_interval: None,
        };
        println!("{}", serde_json::to_string(&req).unwrap());

        let req = super::BatchUpdateFirmwareRequest {
            product_id: "product_id".to_string().into(),
            firmware_version: "firmware_version".to_string().into(),
            firmware_ori_version: "firmware_ori_version".to_string().into(),
            upgrade_method: 1.into(),
            file_name: "file_name".to_string().into(),
            file_md5: "file_md5".to_string().into(),
            file_size: 144.into(),
            device_names: vec!["device0".to_string(), "device1".to_string()].into(),
            timeout_interval: 20.into(),
        };
        println!("{}", serde_json::to_string(&req).unwrap());
    }
}
