use std::sync::Arc;

use reqwest::Method;

use crate::request::Scheme;

#[derive(Debug, Clone)]
pub enum SignMethod {
    Tc3HmacSha256,
}

impl Default for SignMethod {
    fn default() -> Self {
        SignMethod::Tc3HmacSha256
    }
}

impl AsRef<str> for SignMethod {
    fn as_ref(&self) -> &str {
        match self {
            SignMethod::Tc3HmacSha256 => "TC3-HMAC-SHA256",
        }
    }
}

#[derive(Debug, Clone)]
pub enum Language {
    ZHCN,
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        match self {
            Language::ZHCN => "zh-CN",
        }
    }
}

#[derive(Debug, Default)]
pub struct Profile {
    pub client: ClientProfile,
    pub http: HTTProfile,
}

impl Profile {
    pub fn new(client: ClientProfile, http: HTTProfile) -> Arc<Profile> {
        Arc::new(Profile { client, http })
    }
}

#[derive(Debug)]
pub struct ClientProfile {
    pub sing_method: SignMethod,
    pub unsigned_payload: bool,
    pub language: String,
    pub debug: bool,
    pub disable_region_breaker: bool,
    pub backup_endpoint: String,
}

impl Default for ClientProfile {
    fn default() -> Self {
        Self {
            sing_method: Default::default(),
            unsigned_payload: Default::default(),
            language: Default::default(),
            debug: Default::default(),
            disable_region_breaker: Default::default(),
            backup_endpoint: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct HTTProfile {
    pub method: reqwest::Method,
    pub timeout: u64,
    pub scheme: Scheme,
    pub root_domain: String,
    pub endpoint: String,
}

impl Default for HTTProfile {
    fn default() -> Self {
        Self {
            method: Method::POST,
            timeout: 60,
            scheme: Default::default(),
            root_domain: Default::default(),
            endpoint: Default::default(),
        }
    }
}
