use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub device_id: String,
    pub device_name: String,
    pub server_url: String,
    pub reconnect_interval: u64,
    pub heartbeat_interval: u64,
}

impl ClientConfig {
    pub fn new(server_url: String, device_name: Option<String>) -> Result<Self> {
        // Generate or load device ID
        let device_id = Self::get_or_create_device_id()?;
        
        // Determine device name
        let device_name = device_name.unwrap_or_else(|| {
            env::var("COMPUTERNAME")
                .or_else(|_| env::var("HOSTNAME"))
                .unwrap_or_else(|_| "Unknown Device".to_string())
        });
        
        Ok(ClientConfig {
            device_id,
            device_name,
            server_url,
            reconnect_interval: 30, // seconds
            heartbeat_interval: 30, // seconds
        })
    }
    
    fn get_or_create_device_id() -> Result<String> {
        // In a real implementation, this would:
        // 1. Check for existing device ID in registry/config file
        // 2. Generate new one if not found
        // 3. Store it persistently
        
        // For now, generate a new UUID each time
        // TODO: Implement persistent storage
        Ok(Uuid::new_v4().to_string())
    }
}
