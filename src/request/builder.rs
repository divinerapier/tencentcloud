use std::{
    collections::HashMap,
    iter::FromIterator,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::{
    client::{Client, Configuration},
    Credential, Flat, Language, Profile, Region, Scheme, SignMethod, ROOT_DOMAIN,
};
use reqwest::{header::HeaderMap, Method};
use sha2::Digest;
use sha2::Sha256;

use super::ServiceRequest;

#[derive(Debug)]
pub struct RequestBuilder<T> {
    pub method: reqwest::Method,
    pub scheme: Scheme,
    pub root_domain: String,
    pub domain: Option<String>,
    pub path: String,
    pub params: HashMap<String, String>,
    pub form_params: Option<HashMap<String, String>>,
    pub service: Option<String>,
    pub version: Option<String>,
    pub action: Option<String>,
    pub credential: Option<Arc<Credential>>,
    pub profile: Option<Arc<Profile>>,
    pub region: Region,
    pub language: Language,
    pub inner: T,

    pub payload: Option<String>,
    pub headers: HeaderMap,
    // pub query_string: Option<String>,
}

impl<T: Default> Default for RequestBuilder<T> {
    fn default() -> Self {
        Self {
            method: Method::POST,
            scheme: Scheme::HTTPS,
            root_domain: ROOT_DOMAIN.to_string(),
            domain: Default::default(),
            path: "/".to_string(),
            params: Default::default(),
            form_params: Default::default(),
            service: Default::default(),
            version: Default::default(),
            action: Default::default(),
            credential: Default::default(),
            profile: Default::default(),
            region: Default::default(),
            language: Default::default(),
            headers: Default::default(),
            // query_string: Default::default(),
            payload: None,
            inner: Default::default(),
        }
    }
}

impl<T> Deref for RequestBuilder<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for RequestBuilder<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> RequestBuilder<T> {
    pub fn set_method(mut self, method: reqwest::Method) -> Self {
        self.method = method;
        self
    }
    pub fn set_scheme(mut self, scheme: Scheme) -> Self {
        self.scheme = scheme;
        self
    }
    pub fn set_root_domain(mut self, root_domain: String) -> Self {
        self.root_domain = root_domain;
        self
    }
    pub fn set_domain(mut self, domain: String) -> Self {
        self.domain = Some(domain);
        self
    }

    // pub fn set_path(mut self, path: String) -> Self {
    //     self.path = Some(path);
    //     self
    // }

    pub fn set_region(mut self, region: String) -> Self {
        self.params.insert("Region".into(), region);
        self
    }

    pub fn set_version(mut self, version: String) -> Self {
        self.params.insert("Version".to_string(), version);
        self
    }

    fn set_nonce(mut self, nonce: i64) -> Self {
        // let v = rand::random::<i64>();
        self.params.insert("Nonce".to_string(), nonce.to_string());
        self
    }

    // fn flatten_request(mut self) -> Self
    // where
    //     T: Into<HashMap<String, String>> + Clone,
    // {
    //     // let inner = self.inner.take();

    //     // if let Some(inner) = inner {
    //     let inner = self.inner.clone();
    //     let hm = inner.into();
    //     for (k, v) in hm {
    //         self.params.insert(k, v);
    //     }
    //     // }
    //     self
    // }

    // fn ensure_scheme(mut self) -> Option<Self> {
    //     if self.scheme.is_some() {
    //         return Some(self);
    //     }

    //     // let mut request = self.request;
    //     let profile = self.profile.as_ref()?;
    //     let http_profile = &profile.http;
    //     self.scheme = http_profile.scheme;
    //     // self.request = request;
    //     Some(self)
    // }

    // fn ensure_root_domain(mut self) -> Option<Self> {
    //     if self.scheme.is_some() {
    //         return Some(self);
    //     }

    //     // let mut request = self.request;
    //     let profile = self.profile.as_ref().unwrap();
    //     let http_profile = &profile.http;
    //     self.root_domain = Some(http_profile.root_domain.clone());
    //     // self.request = request;
    //     Some(self)
    // }

    fn ensure_domain(mut self) -> Option<Self> {
        if self.domain.is_some() {
            return Some(self);
        }
        // let mut request = self.request;
        let http_profile = &self.profile.clone().unwrap().http;
        let domain = if !http_profile.endpoint.is_empty() {
            http_profile.endpoint.clone()
        } else {
            let service = self.service.as_ref().unwrap();
            // self.get_service_domain(service)
            format!("{}.{}", service, self.root_domain)
        };
        self.domain = Some(domain);
        // self.request = request;
        Some(self)
    }

    // fn ensure_http_method(mut self) -> Self {
    //     if self.method.is_some() {
    //         return self;
    //     }
    //     self.method = Some(Method::GET);
    //     self
    // }

    fn ensure_params(mut self) -> Option<Self> {
        let region = self.region.to_string();
        self.params.insert("Region".to_string(), region);
        if let Some(version) = self.version.as_ref() {
            let version = version.to_string();
            self.params.insert("Version".to_string(), version);
        }
        self.params.insert(
            "Timestamp".to_string(),
            // "1632642247".to_string(),
            chrono::Local::now().timestamp().to_string(),
        );
        self.params
            .insert("RequestClient".to_string(), "SDK_GO_1.0.222".to_string());
        Some(self)
    }

    pub fn ensure(self) -> Option<Self>
    where
        T: ServiceRequest + Flat + serde::Serialize,
    {
        let req = self
            .set_service()?
            .set_action()?
            // .ensure_root_domain()
            // .unwrap()
            .ensure_domain()?
            .set_nonce(rand::random::<i64>())
            .ensure_params()?
            .ensure_headers()
            // .ensure_query()
            .ensure_sign()?;
        Some(req)
    }

    fn ensure_headers(mut self) -> Self {
        self.headers
            .insert("Host", self.domain.as_ref().unwrap().parse().unwrap());
        self.headers.insert(
            "X-TC-Action",
            self.action.as_ref().unwrap().parse().unwrap(),
        );
        self.headers.insert(
            "X-TC-Version",
            self.version.as_ref().unwrap().parse().unwrap(),
        );
        self.headers.insert(
            "X-TC-Timestamp",
            self.params.get("Timestamp").unwrap().parse().unwrap(),
        );
        self.headers.insert(
            "X-TC-RequestClient",
            self.params.get("RequestClient").unwrap().parse().unwrap(),
        );
        self.headers
            .insert("X-TC-Language", self.language.as_ref().parse().unwrap());
        if self.method == Method::GET {
            self.headers.insert(
                "Content-Type",
                "application/x-www-form-urlencoded".parse().unwrap(),
            );
        } else {
            self.headers
                .insert("Content-Type", "application/json".parse().unwrap());
        }
        self
    }

    // fn ensure_query(mut self) -> Self
    // where
    //     T: Flat,
    // {
    //     if self.method != Method::GET {
    //         return self;
    //     }
    //     let map = self.inner.flat();
    //     self.params.extend(map);
    //     self.params.remove("Action");
    //     self.params.remove("Version");
    //     self.params.remove("Nonce");
    //     self.params.remove("Region");
    //     self.params.remove("RequestClient");
    //     self.params.remove("Timestamp");

    //     let mut values = url::query::Values::default();
    //     for (k, v) in &self.params {
    //         values.add(k.into(), v.into());
    //     }
    //     self.query_string = Some(values.encode());
    //     self
    // }

    fn ensure_sign(mut self) -> Option<Self>
    where
        T: serde::Serialize,
    {
        let canonical_headers = format!(
            "content-type:{}\nhost:{}\n",
            self.headers.get("Content-Type").unwrap().to_str().unwrap(),
            self.headers.get("Host").unwrap().to_str().unwrap()
        );
        let signed_headers = "content-type;host";
        // let mut request_payload = "".to_string();
        if self.method == Method::POST {
            self.payload = Some(serde_json::to_string(&self.inner).unwrap());
        }
        //   = "".to_string();
        let mut hashed_request_payload = if self.profile.as_ref()?.client.unsigned_payload {
            self.headers
                .insert("X-TC-Content-SHA256", "UNSIGNED-PAYLOAD".parse().unwrap());
            sha256hex("UNSIGNED-PAYLOAD")
        } else {
            let payload = if let Some(ref payload) = self.payload {
                payload
            } else {
                ""
            };
            // println!("payload: {}", payload);
            sha256hex(payload)
            // println!("hashed_request_payload: {}", hashed_request_payload);
        };
        // let query_string = if let Some(ref query_string) = self.query_string {
        //     query_string
        // } else {
        //     ""
        // };
        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            self.method.as_str(),
            "/",
            "",
            canonical_headers,
            signed_headers,
            hashed_request_payload
        );
        // println!("canonical_request: {}", canonical_request);
        let algorithm = SignMethod::Tc3HmacSha256;
        let request_timestamp = self.headers.get("X-TC-Timestamp").unwrap();
        let reqeust_timestamp =
            unsafe { std::str::from_utf8_unchecked(request_timestamp.as_ref()) };
        let timestamp = reqeust_timestamp.parse::<i64>().unwrap_or(0);
        let t = chrono::NaiveDateTime::from_timestamp_opt(timestamp, 0);
        let date = match t {
            Some(t) => t.format("%Y-%m-%d").to_string(),
            None => "1970-01-01".to_string(),
        };
        let credential_scope = format!("{}/{}/tc3_request", date, self.service.as_ref().unwrap());
        let hashed_canonical_request = sha256hex(&canonical_request);
        // println!("hashed_canonical_request: {}", hashed_canonical_request);
        let string_2_sign = format!(
            "{}\n{}\n{}\n{}",
            algorithm.as_ref(),
            reqeust_timestamp,
            credential_scope,
            hashed_canonical_request
        );
        // println!("string_2_sign: {}", string_2_sign);
        // println!("secret_date. date: {} {:?}", date, date.as_bytes());
        let tc3_secret_key = || {
            let mut tc3 = "TC3".as_bytes().to_vec();
            tc3.extend_from_slice(self.credential.as_ref().unwrap().secret_key().as_bytes());
            tc3
        };
        let secret_date = hmacsha256(
            date.as_bytes(),
            // self.credential.as_ref().unwrap().secret_key().as_bytes(),
            &tc3_secret_key(),
        );
        // println!("secret_service",);
        let secret_service = hmacsha256(self.service.as_ref().unwrap().as_bytes(), &secret_date);
        // println!("secret_key");
        let secret_key = hmacsha256(b"tc3_request", &secret_service);
        let signature = hex::encode(&hmacsha256(string_2_sign.as_bytes(), &secret_key));
        // println!("signature: {:?}", signature);
        let authorization = format!(
            "{} Credential={}/{}, SignedHeaders={}, Signature={}",
            algorithm.as_ref(),
            self.credential.as_ref().unwrap().access_key(),
            credential_scope,
            signed_headers,
            signature
        );
        self.headers
            .insert("Authorization", authorization.parse().unwrap());

        // let u = format!(
        //     "{}://{}{}",
        //     self.scheme.as_ref(),
        //     self.domain.as_ref().unwrap(),
        //     self.path
        // );

        Some(self)
    }
}

pub fn sha256hex<S: AsRef<str>>(s: S) -> String {
    let mut hasher = Sha256::new();
    hasher.update(s.as_ref());
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn hmacsha256(s: &[u8], key: &[u8]) -> Vec<u8> {
    use hmac::{Hmac, Mac, NewMac};

    // Create alias for HMAC-SHA256
    type HmacSha256 = Hmac<Sha256>;

    // Create HMAC-SHA256 instance which implements `Mac` trait
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(s);

    // `result` has type `Output` which is a thin wrapper around array of
    // bytes for providing constant time equality check
    let result = mac.finalize();
    // To get underlying array use `into_bytes` method, but be careful, since
    // incorrect use of the code value may permit timing attacks which defeat
    // the security provided by the `Output`

    let code_bytes = result.into_bytes();
    let slice = code_bytes.as_slice();
    let mut result = Vec::new();
    result.extend(slice);
    // println!("key: {:?}", key);
    // println!("s: {:?}", s);
    // println!("result: {:?}", result);
    result
}

impl<T> RequestBuilder<T>
where
    T: ServiceRequest,
{
    pub fn set_action(mut self) -> Option<Self> {
        let action = self.inner.action().to_string();
        self.action = Some(action.clone());
        self.params.insert("Action".to_string(), action);
        Some(self)
    }

    pub fn set_service(mut self) -> Option<Self> {
        let service = self.inner.service().to_string();
        self.service = Some(service);
        Some(self)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_hexencode() {
        assert_eq!("48656c6c6f", hex::encode(b"Hello"));
        assert_eq!("68656c6c6f20776f726c6421", hex::encode(b"hello world!"))
    }

    #[test]
    fn test_hmacsha256() {
        use super::hmacsha256;

        fn foo(s: &[u8], key: &[u8], expect: &[u8]) {
            let results = hmacsha256(s, key);
            let s = unsafe { std::str::from_utf8_unchecked(s) };
            let key = unsafe { std::str::from_utf8_unchecked(key) };
            println!(
                "key: {}\ts: {}\tresult: {:?}\tlen: {}",
                key,
                s,
                results,
                results.len()
            );
            assert_eq!(&results, expect);
        }

        foo(
            b"data",
            b"key",
            &[
                80, 49, 254, 61, 152, 156, 109, 21, 55, 160, 19, 250, 110, 115, 157, 162, 52, 99,
                253, 174, 195, 183, 1, 55, 216, 40, 227, 106, 206, 34, 27, 208,
            ],
        );
        foo(
            b"data1",
            b"key",
            &[
                149, 121, 145, 230, 94, 153, 207, 231, 108, 102, 47, 164, 125, 90, 146, 62, 251,
                191, 103, 146, 226, 18, 206, 81, 222, 2, 101, 89, 131, 138, 176, 141,
            ],
        );
        foo(
            b"data2",
            b"key",
            &[
                213, 159, 224, 110, 131, 57, 3, 167, 168, 244, 0, 211, 37, 70, 83, 1, 23, 13, 220,
                17, 136, 227, 29, 186, 146, 97, 119, 186, 18, 59, 69, 230,
            ],
        );
        foo(
            b"data3",
            b"key",
            &[
                218, 238, 30, 86, 13, 234, 64, 11, 159, 114, 72, 235, 121, 7, 145, 138, 143, 167,
                173, 158, 67, 48, 26, 17, 204, 120, 239, 188, 63, 22, 154, 89,
            ],
        );
        foo(
            b"data4",
            b"key",
            &[
                170, 249, 59, 97, 78, 30, 126, 107, 253, 219, 242, 25, 212, 45, 246, 219, 185, 235,
                243, 1, 191, 66, 246, 246, 76, 36, 64, 193, 135, 26, 69, 162,
            ],
        );
        foo(
            b"data5",
            b"key",
            &[
                248, 190, 59, 90, 112, 102, 236, 205, 50, 194, 78, 122, 162, 106, 128, 196, 161,
                234, 187, 249, 97, 188, 138, 94, 253, 210, 112, 224, 130, 67, 33, 216,
            ],
        );
    }
}
