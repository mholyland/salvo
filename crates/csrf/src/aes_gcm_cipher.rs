use aead::generic_array::GenericArray;
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::Aes256Gcm;

use super::CsrfCipher;

/// AesGcmCipher is a CSRF protection implementation that uses HMAC.
pub struct AesGcmCipher {
    aead_key: Vec<u8>,
    token_len: usize,
}

impl AesGcmCipher {
    /// Given an HMAC key, return an `AesGcmCipher` instance.
    #[inline]
    pub fn new(aead_key: impl Into<Vec<u8>>) -> Self {
        Self {
            aead_key: aead_key.into(),
            token_len: 32,
        }
    }

    /// Set the length of the token.
    #[inline]
    pub fn with_token_len(mut self, token_len: usize) -> Self {
        assert!(token_len >= 8, "length must be larger than 8");
        self.token_len = token_len;
        self
    }

    #[inline]
    fn aead(&self) -> Aes256Gcm {
        let key = GenericArray::clone_from_slice(&self.aead_key);
        Aes256Gcm::new(&key)
    }
}

impl CsrfCipher for AesGcmCipher {
    fn verify(&self, token: &[u8], secret: &[u8]) -> bool {
        if token.len() < 8 || secret.len() < 16 {
            false
        } else {
            let nonce = GenericArray::from_slice(&secret[0..8]);
            let aead = self.aead();
            aead.decrypt(nonce, &secret[8..]).map(|p| p == token).unwrap_or(false)
        }
    }
    fn generate(&self) -> (Vec<u8>, Vec<u8>) {
        let token = self.random_bytes(self.token_len);
        let aead = self.aead();
        let mut secret = self.random_bytes(8);
        let nonce = GenericArray::from_slice(&secret);
        secret.append(&mut aead.encrypt(nonce, token.as_slice()).unwrap());
        (token, secret)
    }
}