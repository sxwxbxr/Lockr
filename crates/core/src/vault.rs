use anyhow::{Result, anyhow};
use secrecy::Secret;
use serde::{Serialize, Deserialize};
use rand::RngCore;
use crate::crypto::{derive_key, encrypt, decrypt, KeyMaterial};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    pub title: String,
    pub username: String,
    pub password: Secret<String>,
    pub url: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vault {
    pub entries: Vec<Entry>,
    pub salt: [u8; 16],
    pub nonce: [u8; 12],
}

impl Vault {
    pub fn new() -> Self {
        let mut salt = [0u8;16];
        let mut nonce = [0u8;12];
        rand::thread_rng().fill_bytes(&mut salt);
        rand::thread_rng().fill_bytes(&mut nonce);
        Self { entries: vec![], salt, nonce }
    }

    pub fn add(&mut self, e: Entry) {
        self.entries.push(e);
    }

    pub fn serialize_encrypted(&self, master: &Secret<String>) -> Result<Vec<u8>> {
        let key = derive_key(master, &self.salt)?;
        let json = serde_json::to_vec(&self.sanitize())?;
        let ct = encrypt(&key, &json, &self.nonce)?;
        Ok(ct)
    }

    pub fn deserialize_encrypted(bytes: &[u8], master: &Secret<String>, salt: [u8;16], nonce: [u8;12]) -> Result<Self> {
        let key = derive_key(master, &salt)?;
        let pt = decrypt(&key, bytes, &nonce)?;
        let mut v: Vault = serde_json::from_slice(&pt)?;
        v.salt = salt;
        v.nonce = nonce;
        Ok(v)
    }

    fn sanitize(&self) -> Self {
        // clone is fine for placeholder
        self.clone()
    }
}
