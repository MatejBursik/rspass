use crate::encrypt::EncryptedData;

use anyhow::{Context, Result};
use dirs::home_dir;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use zeroize::Zeroize;

#[derive(Serialize, Deserialize, Default)]
pub struct VaultData {
    pub entries: HashMap<String, String> // service -> password
}

impl Zeroize for VaultData {
    fn zeroize(&mut self) {
        for (_, password) in self.entries.iter_mut() {
            password.zeroize();
        }

        self.entries.clear();
    }
}

impl Drop for VaultData {
    fn drop(&mut self) {
        self.zeroize();
    }
}

pub struct Vault {
    data: VaultData,
    file_path: PathBuf
}

impl Vault {
    // Get the default vault file path
    fn get_vault_path() -> Result<PathBuf> {
        let home_dir = home_dir().context("Could not find home directory")?;
        
        let config_dir = home_dir.join(".config").join("rspass");
        fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
        
        Ok(config_dir.join("vault.enc"))
    }
    
    // Create a new empty vault
    pub fn create_new(master_password: &str) -> Result<()> {
        let vault_path = Self::get_vault_path()?;
        
        if vault_path.exists() {
            anyhow::bail!("Vault already exists. Use other commands to manage it.");
        }
        
        let vault = Vault {
            data: VaultData::default(),
            file_path: vault_path
        };
        
        vault.save(master_password)?;
        
        Ok(())
    }

    // Save vault to disk
    pub fn save(&self, master_password: &str) -> Result<()> {
        let json_data = serde_json::to_vec(&self.data).context("Failed to serialize vault data")?;
        
        let encrypted_data = EncryptedData::encrypt(&json_data, master_password).context("Failed to encrypt vault data")?;
        
        let encrypted_json = serde_json::to_vec_pretty(&encrypted_data).context("Failed to serialize encrypted data")?;
        
        fs::write(&self.file_path, encrypted_json).context("Failed to write vault file")?;
        
        Ok(())
    }
}
