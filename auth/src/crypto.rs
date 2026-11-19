//! Encrypts account tokens before they are written to disk.
//!
//! A random 256-bit master key lives in the OS credential store (Windows Credential
//! Manager / macOS Keychain / Linux Secret Service). Tokens are encrypted with
//! ChaCha20-Poly1305 using that key, and the result is stored as
//! `base64(nonce || ciphertext)` in the account TOML file. Only the key goes into the
//! credential store because Windows limits each credential to 2560 bytes, which
//! Microsoft refresh tokens can exceed.

use base64::{Engine, engine::general_purpose::STANDARD};
use chacha20poly1305::{
    ChaCha20Poly1305, Nonce,
    aead::{Aead, Generate, Key, KeyInit},
};

use crate::error::Error;

const KEYRING_SERVICE: &str = "TSMCL";
const KEYRING_USER: &str = "account-encryption-key";
const NONCE_LEN: usize = 12;

fn keyring_entry() -> Result<keyring::Entry, Error> {
    Ok(keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER)?)
}

/// Load the master key from the credential store, creating one on first use.
fn cipher() -> Result<ChaCha20Poly1305, Error> {
    let entry = keyring_entry()?;
    match entry.get_secret() {
        Ok(secret) => ChaCha20Poly1305::new_from_slice(&secret).map_err(|_| Error::InvalidKey),
        Err(keyring::Error::NoEntry) => {
            let key = Key::<ChaCha20Poly1305>::generate();
            entry.set_secret(&key)?;
            Ok(ChaCha20Poly1305::new(&key))
        }
        Err(e) => Err(e.into()),
    }
}

pub(crate) fn encrypt(plaintext: &str) -> Result<String, Error> {
    let nonce = Nonce::generate();
    let ciphertext = cipher()?
        .encrypt(&nonce, plaintext.as_bytes())
        .map_err(|_| Error::Encrypt)?;

    let mut out = Vec::with_capacity(NONCE_LEN + ciphertext.len());
    out.extend_from_slice(&nonce);
    out.extend_from_slice(&ciphertext);
    Ok(STANDARD.encode(out))
}

pub(crate) fn decrypt(encoded: &str) -> Result<String, Error> {
    let data = STANDARD.decode(encoded).map_err(|_| Error::Decrypt)?;
    if data.len() < NONCE_LEN {
        return Err(Error::Decrypt);
    }
    let (nonce, ciphertext) = data.split_at(NONCE_LEN);
    let nonce = Nonce::try_from(nonce).map_err(|_| Error::Decrypt)?;
    let plaintext = cipher()?
        .decrypt(&nonce, ciphertext)
        .map_err(|_| Error::Decrypt)?;
    String::from_utf8(plaintext).map_err(|_| Error::Decrypt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() -> Result<(), Error> {
        let encrypted = encrypt("secret-token")?;
        assert_ne!(encrypted, "secret-token");
        assert_eq!(decrypt(&encrypted)?, "secret-token");
        Ok(())
    }

    #[test]
    fn tampered_data_fails() -> Result<(), Error> {
        let mut data = STANDARD.decode(encrypt("secret-token")?).unwrap();
        *data.last_mut().unwrap() ^= 1;
        assert!(decrypt(&STANDARD.encode(data)).is_err());
        Ok(())
    }
}
