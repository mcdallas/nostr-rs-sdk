// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::str::FromStr;

use bitcoin::secp256k1::XOnlyPublicKey;
use reqwest::blocking::Client;
use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid format")]
    InvalidFormat,
    #[error("impossible to verify")]
    ImpossibleToVerify,
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    /// Error serializing or deserializing JSON data
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("secp256k1 error: {0}")]
    Secp256k1(#[from] bitcoin::secp256k1::Error),
}

/// Verify NIP-05
pub fn verify(public_key: XOnlyPublicKey, nip05: &str) -> Result<(), Error> {
    let data: Vec<&str> = nip05.split('@').collect();
    if data.len() != 2 {
        return Err(Error::InvalidFormat);
    }

    let name: &str = data[0];
    let domain: &str = data[1];

    let url = format!("https://{}/.well-known/nostr.json?name={}", domain, name);

    let req = Client::new().get(url);

    let res = req.send()?;
    let json: Value = serde_json::from_str(&res.text()?)?;

    if let Some(names) = json.get("names") {
        if let Some(value) = names.get(name) {
            if let Some(pubkey) = value.as_str() {
                if XOnlyPublicKey::from_str(pubkey)? == public_key {
                    return Ok(());
                }
            }
        }
    }

    Err(Error::ImpossibleToVerify)
}
