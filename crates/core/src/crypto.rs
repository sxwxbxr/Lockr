use anyhow::Result;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{PasswordHash, PasswordHasher as _, PasswordVerifier, SaltString};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use rand::rngs::OsRng;
use secrecy::{Secret, ExposeSecret};
use zeroize::Zeroize;

#[derive(Clone)]
pub struct KeyMaterial {
    key: Secret<[u8; 32]>,
}

impl Drop for KeyMaterial {
    fn drop(&mut self) {
        // zeroize handled by Secret on drop
    }
}

pub fn derive_key(master: &Secret<String>, salt: &[u8]) -> Result<KeyMaterial> {
    let argon = Argon2::default();
    let mut output = [0u8; 32];
    argon.hash_password_into(master.expose_secret().as_bytes(), salt, &mut output)?;
    Ok(KeyMaterial { key: Secret::new(output) })
}

pub fn encrypt(key: &KeyMaterial, plaintext: &[u8], nonce_bytes: &[u8;12]) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key.key.expose_secret()));
    let nonce = Nonce::from_slice(nonce_bytes);
    let ct = cipher.encrypt(nonce, plaintext)?;
    Ok(ct)
}

pub fn decrypt(key: &KeyMaterial, ciphertext: &[u8], nonce_bytes: &[u8;12]) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key.key.expose_secret()));
    let nonce = Nonce::from_slice(nonce_bytes);
    let pt = cipher.decrypt(nonce, ciphertext)?;
    Ok(pt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use secrecy::Secret;

    #[test]
    fn roundtrip() {
        let key = derive_key(&Secret::new("test".to_string()), b"saltsaltsaltsalt").unwrap();
        let nonce = [1u8;12];
        let msg = b"hello";
        let ct = encrypt(&key, msg, &nonce).unwrap();
        let pt = decrypt(&key, &ct, &nonce).unwrap();
        assert_eq!(pt, msg);
    }
}
