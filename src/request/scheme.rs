use std::{convert::TryFrom, str::FromStr};

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

impl FromStr for Scheme {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "http" => Ok(Scheme::HTTP),
            "https" => Ok(Scheme::HTTPS),
            // TODO: handle error
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for Scheme {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_ref() {
            "http" => Ok(Scheme::HTTP),
            "https" => Ok(Scheme::HTTPS),
            // TODO: handle error
            _ => Err(()),
        }
    }
}
