use std::{borrow::Cow, collections::HashMap, convert::TryFrom, sync::Arc};

use reqwest::Method;

use crate::{credential::Credential, profile::Profile, region::Region};

const ROOT_DOMAIN: &str = "tencentcloudapi.com";
const API_VERSION: &str = "2018-06-14";

#[derive(Debug, Clone, Copy)]
pub enum Scheme {
    HTTP,
    HTTPS,
}

impl Default for Scheme {
    fn default() -> Self {
        Scheme::HTTPS
    }
}

impl AsRef<str> for Scheme {
    fn as_ref(&self) -> &str {
        match self {
            Scheme::HTTP => "http",
            Scheme::HTTPS => "https",
        }
    }
}

impl TryFrom<&str> for Scheme {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "http" | "HTTP" => Ok(Scheme::HTTP),
            "https" | "HTTPS" => Ok(Scheme::HTTPS),
            _ => match value.to_lowercase().as_ref() {
                "http" => Ok(Scheme::HTTP),
                "https" => Ok(Scheme::HTTPS),
                _ => Err(()),
            },
        }
    }
}

pub trait ServiceRequest {
    fn service(&self) -> &'static str;
    fn action(&self) -> &'static str;
}

pub struct Request<T> {
    pub method: Option<reqwest::Method>,
    pub scheme: Option<Scheme>,
    pub root_domain: Option<String>,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub params: HashMap<String, String>,
    pub form_params: HashMap<String, String>,
    pub service: Option<String>,
    pub version: Option<String>,
    pub action: Option<String>,
    pub credential: Option<Arc<Credential>>,
    pub profile: Option<Arc<Profile>>,
    pub region: Option<Arc<Region>>,
    inner: Option<T>,
}

impl<T> Request<T> {
    fn get_service_domain(&self, service: &str) -> String {
        let mut root_domain = "";
        if let Some(rd) = self.root_domain.as_ref() {
            root_domain = rd;
        }
        if root_domain.is_empty() {
            root_domain = ROOT_DOMAIN;
        }
        format!("{}.{}", service, root_domain)
    }
}

impl<T: Default> Default for Request<T> {
    fn default() -> Self {
        Self {
            method: Default::default(),
            scheme: Default::default(),
            root_domain: Default::default(),
            domain: Default::default(),
            path: Default::default(),
            params: Default::default(),
            form_params: Default::default(),
            service: Default::default(),
            version: Default::default(),
            action: Default::default(),
            credential: Default::default(),
            profile: Default::default(),
            region: Default::default(),
            inner: Default::default(),
        }
    }
}

pub struct RequestBuilder<T> {
    request: Request<T>,
}

impl<T: Default> Default for RequestBuilder<T> {
    fn default() -> Self {
        Self {
            request: Default::default(),
        }
    }
}

impl<T> RequestBuilder<T> {
    pub fn set_method(mut self, method: reqwest::Method) -> Self {
        self.request.method = Some(method);
        self
    }
    pub fn set_scheme(mut self, scheme: Scheme) -> Self {
        self.request.scheme = Some(scheme);
        self
    }
    pub fn set_root_domain(mut self, root_domain: String) -> Self {
        self.request.root_domain = Some(root_domain);
        self
    }
    pub fn set_domain(mut self, domain: String) -> Self {
        self.request.domain = Some(domain);
        self
    }
    pub fn set_path(mut self, path: String) -> Self {
        self.request.path = Some(path);
        self
    }

    pub fn set_region(mut self, region: String) -> Self {
        self.request.params.insert("Region".into(), region);
        self
    }

    pub fn set_version(mut self, version: String) -> Self {
        self.request.params.insert("Version".to_string(), version);
        self
    }

    fn set_nonce(mut self) -> Self {
        let v = rand::random::<i64>();
        self.request
            .params
            .insert("Nonce".to_string(), v.to_string());
        self
    }

    fn flatten_request(mut self) -> Self
    where
        T: Into<HashMap<String, String>>,
    {
        let inner = self.request.inner.take();
        if let Some(inner) = inner {
            let hm = inner.into();
            for (k, v) in hm {
                self.request.params.insert(k, v);
            }
        }
        self
    }

    pub fn build(self) -> Option<Request<T>>
    where
        T: Into<HashMap<String, String>> + ServiceRequest,
    {
        let request = self
            .set_nonce()
            .set_action()?
            .set_service()?
            .ensure_scheme()?
            .ensure_root_domain()?
            .ensure_domain()?
            .ensure_http_method()
            .ensure_params()?
            .flatten_request()
            .request;

        Some(request)
    }

    fn ensure_scheme(mut self) -> Option<Self> {
        if self.request.scheme.is_some() {
            return Some(self);
        }

        let mut request = self.request;
        let profile = request.profile.as_ref()?;
        let http_profile = &profile.http;
        request.scheme = Some(http_profile.scheme);
        self.request = request;
        Some(self)
    }

    fn ensure_root_domain(mut self) -> Option<Self> {
        if self.request.scheme.is_some() {
            return Some(self);
        }

        let mut request = self.request;
        let profile = request.profile.as_ref()?;
        let http_profile = &profile.http;
        request.root_domain = Some(http_profile.root_domain.clone());
        self.request = request;
        Some(self)
    }

    fn ensure_domain(mut self) -> Option<Self> {
        if self.request.domain.is_some() {
            return Some(self);
        }
        let mut request = self.request;
        let http_profile = &request.profile.clone()?.http;
        let domain = if !http_profile.endpoint.is_empty() {
            http_profile.endpoint.clone()
        } else {
            let service = request.service.as_ref()?;
            request.get_service_domain(service)
        };
        request.domain = Some(domain);
        self.request = request;
        Some(self)
    }

    fn ensure_http_method(mut self) -> Self {
        if self.request.method.is_some() {
            return self;
        }
        self.request.method = Some(Method::GET);
        self
    }

    fn ensure_params(mut self) -> Option<Self> {
        let region = self.request.region.as_ref()?.to_string();
        self.request.params.insert("Region".to_string(), region);
        if let Some(version) = self.request.version.as_ref() {
            let version = version.to_string();
            self.request.params.insert("Version".to_string(), version);
        }
        self.request.params.insert(
            "Timestamp".to_string(),
            chrono::Local::now().timestamp().to_string(),
        );
        self.request
            .params
            .insert("RequestClient".to_string(), "Rust".to_string());
        Some(self)
    }
}

impl<T> RequestBuilder<T>
where
    T: ServiceRequest,
{
    pub fn set_action(mut self) -> Option<Self> {
        let inner = self.request.inner.as_ref()?;
        self.request.action = Some(inner.action().to_string());
        self.request
            .params
            .insert("Action".to_string(), inner.action().to_string());
        Some(self)
    }

    pub fn set_service(mut self) -> Option<Self> {
        let inner = self.request.inner.as_ref()?;
        self.request.service = Some(inner.service().to_string());
        Some(self)
    }
}

#[derive(Default)]
pub struct BatchUpdateFirmwareRequest {
    pub product_id: Option<String>,
    pub firmware_version: Option<String>,
    pub firmware_ori_version: Option<String>,
    pub upgrade_method: Option<u64>,
    pub file_name: Option<String>,
    pub file_md5: Option<String>,
    pub file_size: Option<u64>,
    pub device_names: Option<Vec<String>>,
    pub timeout_interval: Option<u64>,
}

impl ServiceRequest for BatchUpdateFirmwareRequest {
    fn service(&self) -> &'static str {
        "iotcloud"
    }

    fn action(&self) -> &'static str {
        "BatchUpdateFirmware"
    }
}

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

    pub fn into_request(
        self,
        cred: Arc<Credential>,
        prof: Arc<Profile>,
        region: Arc<Region>,
    ) -> RequestBuilder<BatchUpdateFirmwareRequest> {
        RequestBuilder {
            request: Request {
                credential: cred.into(),
                profile: prof.into(),
                region: region.into(),
                version: Some(API_VERSION.to_string()),
                ..Default::default()
            },
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

fn insert<T: ToString>(hm: &mut HashMap<String, String>, key: &str, value: Option<T>) {
    if let Some(value) = value {
        hm.insert(key.to_string(), value.to_string());
    }
}

impl From<BatchUpdateFirmwareRequest> for HashMap<String, String> {
    fn from(mut r: BatchUpdateFirmwareRequest) -> Self {
        let mut hm = HashMap::new();
        insert(&mut hm, "ProductID", r.product_id.take());
        insert(&mut hm, "FirmwareVersion", r.firmware_version.take());
        insert(&mut hm, "FirmwareOriVersion", r.firmware_ori_version.take());
        insert(&mut hm, "UpgradeMethod", r.upgrade_method.take());
        insert(&mut hm, "FileName", r.file_name.take());
        insert(&mut hm, "FileMd5", r.file_md5.take());
        insert(&mut hm, "FileSize", r.file_size.take());
        if let Some(device_names) = r.device_names.as_ref() {
            for (index, device_name) in device_names.iter().enumerate() {
                insert(
                    &mut hm,
                    &format!("DeviceNames.{}", index),
                    Some(device_name),
                );
            }
        }
        insert(&mut hm, "TimeoutInterval", r.timeout_interval.take());
        hm
    }
}
