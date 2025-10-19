use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm
};
use anyhow::{Context, Result};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::RngCore, SaltString};
use zeroize::ZeroizeOnDrop;

#[derive(Clone, ZeroizeOnDrop)]
pub struct EncryptionKey([u8; 32]);

impl EncryptionKey {
    // Derive an encryption key from a master password using Argon2
    pub fn derive_from_password(password: &str, salt: &[u8]) -> Result<Self> {
        let argon2 = Argon2::default();
        let salt_string = SaltString::encode_b64(salt)
            .map_err(|e| anyhow::anyhow!(e.to_string()))
            .context("Failed to encode salt")?;
        
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt_string)
            .map_err(|e| anyhow::anyhow!(e.to_string()))
            .context("Failed to hash password")?;
        
        let hash = password_hash.hash.context("No hash in password hash")?;

        let hash_bytes = hash.as_bytes();
        
        if hash_bytes.len() < 32 {
            anyhow::bail!("Hash too short");
        }
        
        let mut key = [0u8; 32];
        key.copy_from_slice(&hash_bytes[..32]);
        
        Ok(EncryptionKey(key))
    }
    
    // Generate a random salt for key derivation
    pub fn generate_salt() -> [u8; 32] {
        let mut salt = [0u8; 32];
        OsRng.fill_bytes(&mut salt);

        salt
    }
    
    // Verify a password against a stored hash
    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| anyhow::anyhow!(e.to_string()))
            .context("Failed to parse password hash")?;
        
        let argon2 = Argon2::default();
        
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
    
    // Create a password hash for storage
    pub fn hash_password(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!(e.to_string()))
            .context("Failed to hash password")?;
        
        Ok(password_hash.to_string())
    }
}

impl AsRef<[u8]> for EncryptionKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EncryptedData {
    pub nonce: Vec<u8>,
    pub ciphertext: Vec<u8>,
    pub salt: Vec<u8>,
    pub password_hash: String
}

impl EncryptedData {
    // Encrypt data using AES-GCM
    pub fn encrypt(data: &[u8], password: &str) -> Result<Self> {
        // Generate salt and derive key
        let salt = EncryptionKey::generate_salt();
        let key = EncryptionKey::derive_from_password(password, &salt)?;
        
        // Create cipher
        let cipher = Aes256Gcm::new(key.as_ref().into());
        
        // Generate nonce
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        
        // Encrypt
        let ciphertext = cipher
            .encrypt(&nonce, data)
            .map_err(|e| anyhow::anyhow!(e.to_string()))
            .context("Encryption failed")?;
        
        // Hash password for verification
        let password_hash = EncryptionKey::hash_password(password)?;
        
        Ok(EncryptedData {
            nonce: nonce.to_vec(),
            ciphertext,
            salt: salt.to_vec(),
            password_hash
        })
    }
    
    // Decrypt data using AES-GCM
    pub fn decrypt(&self, password: &str) -> Result<Vec<u8>> {
        // Verify password
        if !EncryptionKey::verify_password(password, &self.password_hash)? {
            anyhow::bail!("Invalid master password");
        }
        
        // Derive key
        let key = EncryptionKey::derive_from_password(password, &self.salt)?;
        
        // Create cipher
        let cipher = Aes256Gcm::new(key.as_ref().into());
        
        // Create nonce
        if self.nonce.len() != 12 {
            anyhow::bail!("Invalid nonce length");
        }
        let nonce = self.nonce.as_slice().into();
        
        // Decrypt
        let plaintext = cipher
            .decrypt(nonce, self.ciphertext.as_ref())
            .map_err(|e| anyhow::anyhow!(e.to_string()))
            .context("Decryption failed - wrong password or corrupted data")?;
        
        Ok(plaintext)
    }
}
