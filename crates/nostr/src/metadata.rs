// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use url::Url;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error serializing or deserializing JSON data
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub about: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nip05: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lud06: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lud16: Option<String>,
}

impl Default for Metadata {
    fn default() -> Self {
        Self::new()
    }
}

impl Metadata {
    pub fn new() -> Self {
        Self {
            name: None,
            display_name: None,
            about: None,
            website: None,
            picture: None,
            nip05: None,
            lud06: None,
            lud16: None,
        }
    }

    pub fn from_json<S>(json: S) -> Result<Self, Error>
    where
        S: Into<String>,
    {
        Ok(serde_json::from_str(&json.into())?)
    }

    pub fn as_json(&self) -> Result<String, Error> {
        Ok(serde_json::to_string(&self)?)
    }

    /// Set name
    pub fn name<S>(self, name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: Some(name.into()),
            ..self
        }
    }

    /// Set display_name
    pub fn display_name<S>(self, display_name: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            display_name: Some(display_name.into()),
            ..self
        }
    }

    /// Set about
    pub fn about<S>(self, about: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            about: Some(about.into()),
            ..self
        }
    }

    /// Set website
    pub fn website(self, url: Url) -> Self {
        Self {
            website: Some(url),
            ..self
        }
    }

    /// Set picture
    pub fn picture(self, picture: Url) -> Self {
        Self {
            picture: Some(picture),
            ..self
        }
    }

    /// Set nip05
    pub fn nip05<S>(self, nip05: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            nip05: Some(nip05.into()),
            ..self
        }
    }

    /// Set lud06 (LNURL)
    pub fn lud06<S>(self, lud06: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            lud06: Some(lud06.into()),
            ..self
        }
    }

    /// Set lud16 (Lightning Address)
    pub fn lud16<S>(self, lud16: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            lud16: Some(lud16.into()),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_metadata() {
        let content = r#"{"name":"myname","about":"Description","display_name":""}"#;
        let metadata = Metadata::from_json(content).unwrap();
        assert_eq!(
            metadata,
            Metadata::new()
                .name("myname")
                .about("Description")
                .display_name("")
        );
    }
}
