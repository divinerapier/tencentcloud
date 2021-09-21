use std::sync::Arc;

// FIXME: only support the Credential

#[derive(Default, Debug)]
pub struct Credential {
    access_key: String,
    secret_key: String,
    token: Option<String>,
}

impl Credential {
    pub fn access_key(&self) -> &str {
        &self.access_key
    }
    pub fn secret_key(&self) -> &str {
        &self.secret_key
    }
    pub fn token(&self) -> Option<&str> {
        Some(self.token.as_ref()?)
    }

    pub fn builder() -> CredentialBuilder {
        CredentialBuilder {
            cred: Default::default(),
        }
    }
}

pub struct CredentialBuilder {
    cred: Credential,
}

impl CredentialBuilder {
    pub fn access_key(mut self, access_key: &str) -> Self {
        self.cred.access_key = access_key.to_string();
        self
    }

    pub fn secret_key(mut self, secret_key: &str) -> Self {
        self.cred.secret_key = secret_key.to_string();
        self
    }

    pub fn token(mut self, token: &str) -> Self {
        self.cred.token = Some(token.to_string());
        self
    }

    pub fn build(self) -> Credential {
        self.cred
    }
}
