// Copyright (c) 2021 Paul Miller
// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

use std::str::FromStr;

use bitcoin::bech32::{self, FromBase32, ToBase32, Variant};
use bitcoin::secp256k1::rand::rngs::OsRng;
pub use bitcoin::secp256k1::{KeyPair, Secp256k1, SecretKey, XOnlyPublicKey};

const PREFIX_BECH32_SECRET_KEY: &str = "nsec";
const PREFIX_BECH32_PUBLIC_KEY: &str = "npub";

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("Invalid secret key")]
    InvalidSecretKey,
    #[error("Invalid public key")]
    InvalidPublicKey,
    /// Bech32 encoding error.
    #[error("Bech32 key encoding error: {0}")]
    Bech32(#[from] bech32::Error),
    #[error("Invalid bech32 secret key")]
    Bech32SkParseError,
    #[error("Invalid bech32 public key")]
    Bech32PkParseError,
    #[error("Secrete key missing")]
    SkMissing,
    #[error("Key pair missing")]
    KeyPairMissing,
    #[error("Failed to generate new keys")]
    KeyGenerationFailure,
    /// Secp256k1 error
    #[error("secp256k1 error: {0}")]
    Secp256k1(#[from] bitcoin::secp256k1::Error),
}

pub trait FromSkStr: Sized {
    type Err;
    fn from_sk_str(secret_key: &str) -> Result<Self, Self::Err>;
}

pub trait FromPkStr: Sized {
    type Err;
    fn from_pk_str(public_key: &str) -> Result<Self, Self::Err>;
}

pub trait FromBech32: Sized {
    fn from_bech32<S>(secret_key: S) -> Result<Self, Error>
    where
        S: Into<String>;
    fn from_bech32_public_key<S>(public_key: S) -> Result<Self, Error>
    where
        S: Into<String>;
}

pub trait ToBech32 {
    type Err;
    fn to_bech32(&self) -> Result<String, Self::Err>;
}

impl ToBech32 for XOnlyPublicKey {
    type Err = Error;

    fn to_bech32(&self) -> Result<String, Self::Err> {
        let data = self.serialize().to_base32();
        Ok(bech32::encode(
            PREFIX_BECH32_PUBLIC_KEY,
            data,
            Variant::Bech32,
        )?)
    }
}

impl ToBech32 for SecretKey {
    type Err = Error;

    fn to_bech32(&self) -> Result<String, Self::Err> {
        let data = self.secret_bytes().to_base32();
        Ok(bech32::encode(
            PREFIX_BECH32_SECRET_KEY,
            data,
            Variant::Bech32,
        )?)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Keys {
    public_key: XOnlyPublicKey,
    key_pair: Option<KeyPair>,
    secret_key: Option<SecretKey>,
}

impl Keys {
    /// Initialize from secret key.
    pub fn new(secret_key: SecretKey) -> Self {
        let secp = Secp256k1::new();
        let key_pair = KeyPair::from_secret_key(&secp, &secret_key);
        let public_key = XOnlyPublicKey::from_keypair(&key_pair).0;

        Self {
            public_key,
            key_pair: Some(key_pair),
            secret_key: Some(secret_key),
        }
    }

    /// Initialize with public key only (no secret key).
    pub fn from_public_key(public_key: XOnlyPublicKey) -> Self {
        Self {
            public_key,
            key_pair: None,
            secret_key: None,
        }
    }

    /// Generate a new random keys
    pub fn generate_from_os_random() -> Self {
        let secp = Secp256k1::new();
        let mut rng = OsRng::default();
        let (secret_key, _) = secp.generate_keypair(&mut rng);
        Self::new(secret_key)
    }

    /// Get public key
    pub fn public_key(&self) -> XOnlyPublicKey {
        self.public_key
    }

    /// Get secret key
    pub fn secret_key(&self) -> Result<SecretKey, Error> {
        if let Some(secret_key) = self.secret_key {
            Ok(secret_key)
        } else {
            Err(Error::SkMissing)
        }
    }

    /// Get keypair
    pub fn key_pair(&self) -> Result<KeyPair, Error> {
        if let Some(key_pair) = self.key_pair {
            Ok(key_pair)
        } else {
            Err(Error::KeyPairMissing)
        }
    }

    /// Get secret key as string
    pub fn secret_key_as_str(&self) -> Result<String, Error> {
        Ok(self.secret_key()?.display_secret().to_string())
    }

    /// Get public key as string
    pub fn public_key_as_str(&self) -> String {
        self.public_key.to_string()
    }
}

impl FromSkStr for Keys {
    type Err = Error;

    /// Init [`Keys`] from `hex` or `bech32` secret key
    fn from_sk_str(secret_key: &str) -> Result<Self, Self::Err> {
        match SecretKey::from_str(secret_key) {
            Ok(secret_key) => Ok(Self::new(secret_key)),
            Err(_) => match Self::from_bech32(secret_key) {
                Ok(keys) => Ok(keys),
                Err(_) => Err(Error::InvalidSecretKey),
            },
        }
    }
}

impl FromPkStr for Keys {
    type Err = Error;

    /// Init [`Keys`] from `hex` or `bech32` public key
    fn from_pk_str(public_key: &str) -> Result<Self, Self::Err> {
        match XOnlyPublicKey::from_str(public_key) {
            Ok(public_key) => Ok(Self::from_public_key(public_key)),
            Err(_) => match Self::from_bech32_public_key(public_key) {
                Ok(keys) => Ok(keys),
                Err(_) => Err(Error::InvalidSecretKey),
            },
        }
    }
}

impl FromBech32 for Keys {
    fn from_bech32<S>(secret_key: S) -> Result<Self, Error>
    where
        S: Into<String>,
    {
        let (hrp, data, checksum) =
            bech32::decode(&secret_key.into()).map_err(|_| Error::Bech32SkParseError)?;

        if hrp != PREFIX_BECH32_SECRET_KEY || checksum != Variant::Bech32 {
            return Err(Error::Bech32SkParseError);
        }

        let data = Vec::<u8>::from_base32(&data).map_err(|_| Error::Bech32SkParseError)?;

        let secret_key =
            SecretKey::from_slice(data.as_slice()).map_err(|_| Error::Bech32SkParseError)?;

        let secp = Secp256k1::new();
        let key_pair = KeyPair::from_secret_key(&secp, &secret_key);
        let public_key = XOnlyPublicKey::from_keypair(&key_pair).0;

        Ok(Self {
            public_key,
            key_pair: Some(key_pair),
            secret_key: Some(secret_key),
        })
    }

    fn from_bech32_public_key<S>(public_key: S) -> Result<Self, Error>
    where
        S: Into<String>,
    {
        let (hrp, data, checksum) =
            bech32::decode(&public_key.into()).map_err(|_| Error::Bech32PkParseError)?;

        if hrp != PREFIX_BECH32_PUBLIC_KEY || checksum != Variant::Bech32 {
            return Err(Error::Bech32PkParseError);
        }

        let data = Vec::<u8>::from_base32(&data).map_err(|_| Error::Bech32PkParseError)?;

        let public_key = XOnlyPublicKey::from_slice(data.as_slice())?;

        Ok(Keys {
            public_key,
            key_pair: None,
            secret_key: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    fn to_bech32_public_key() -> Result<()> {
        let bech32_pubkey_str: &str =
            "npub14f8usejl26twx0dhuxjh9cas7keav9vr0v8nvtwtrjqx3vycc76qqh9nsy";
        let keys = Keys::from_bech32_public_key(bech32_pubkey_str)?;
        let public_key: XOnlyPublicKey = keys.public_key();

        assert_eq!(bech32_pubkey_str.to_string(), public_key.to_bech32()?);

        Ok(())
    }

    #[test]
    fn to_bech32_secret_key() -> Result<()> {
        let bech32_secret_key_str: &str =
            "nsec1j4c6269y9w0q2er2xjw8sv2ehyrtfxq3jwgdlxj6qfn8z4gjsq5qfvfk99";
        let keys = Keys::from_bech32(bech32_secret_key_str)?;
        let secret_key: SecretKey = keys.secret_key()?;

        assert_eq!(bech32_secret_key_str.to_string(), secret_key.to_bech32()?);

        Ok(())
    }
}
