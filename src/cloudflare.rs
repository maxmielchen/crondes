use std::error::Error;
use crate::config::Config;

/// Struct for interacting with the Cloudflare API for DNS record management.
///
/// This struct wraps a [`Config`] object and provides methods to check credentials,
/// validate zone and record IDs, fetch the current DNS record IP, and update the record.
pub struct Cloudflare {
    /// The configuration containing API token, zone ID, record ID, and update interval.
    pub config: Config,
}

impl Cloudflare {
    /// Creates a new [`Cloudflare`] instance from the given [`Config`].
    pub fn new(config: Config) -> Self {
        Cloudflare { config }
    }

    /// Checks if the API token is valid by making a test request to the Cloudflare API.
    ///
    /// # Returns
    /// - `Ok(true)` if the token is valid.
    /// - `Ok(false)` if the token is invalid.
    /// - `Err` if the request fails.
    pub async fn api_token_right(&self) -> Result<bool, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let resp = client
            .get("https://api.cloudflare.com/client/v4/user/tokens/verify")
            .bearer_auth(&self.config.cloudflare_api_token)
            .send()
            .await?;
        Ok(resp.status().is_success())
    }

    /// Checks if the zone ID is valid and accessible with the current API token.
    ///
    /// # Returns
    /// - `Ok(true)` if the zone ID is valid and accessible.
    /// - `Ok(false)` if not.
    /// - `Err` if the request fails.
    pub async fn zone_id_right(&self) -> Result<bool, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let url = format!("https://api.cloudflare.com/client/v4/zones/{}", self.config.cloudflare_zone_id);
        let resp = client
            .get(&url)
            .bearer_auth(&self.config.cloudflare_api_token)
            .send()
            .await?;
        Ok(resp.status().is_success())
    }

    /// Checks if the record ID is valid and accessible with the current API token and zone ID.
    ///
    /// # Returns
    /// - `Ok(true)` if the record ID is valid and accessible.
    /// - `Ok(false)` if not.
    /// - `Err` if the request fails.
    pub async fn record_id_right(&self) -> Result<bool, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}", self.config.cloudflare_zone_id, self.config.cloudflare_record_id);
        let resp = client
            .get(&url)
            .bearer_auth(&self.config.cloudflare_api_token)
            .send()
            .await?;
        Ok(resp.status().is_success())
    }

    /// Gets the current IP address set in the DNS record.
    ///
    /// # Returns
    /// - `Ok(ip)` with the current IP as a string if successful.
    /// - `Err` if the request fails or the IP cannot be found.
    pub async fn current_ip(&self) -> Result<String, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}", self.config.cloudflare_zone_id, self.config.cloudflare_record_id);
        let resp = client
            .get(&url)
            .bearer_auth(&self.config.cloudflare_api_token)
            .send()
            .await?;
        let json: serde_json::Value = resp.json().await?;
        let ip = json["result"]["content"].as_str().ok_or("No IP found in record")?;
        Ok(ip.to_string())
    }

    /// Updates the DNS record with a new IP address.
    ///
    /// # Arguments
    /// - `new_ip`: The new IP address to set for the DNS record.
    ///
    /// # Returns
    /// - `Ok(())` if the update was successful.
    /// - `Err` if the update failed.
    pub async fn update_ip(&self, new_ip: &str) -> Result<(), Box<dyn Error>> {
        let client = reqwest::Client::new();
        let url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}", self.config.cloudflare_zone_id, self.config.cloudflare_record_id);
        let body = serde_json::json!({
            "type": "A",
            "name": "",
            "content": new_ip,
            "ttl": 1,
            "proxied": false
        });
        let resp = client
            .put(&url)
            .bearer_auth(&self.config.cloudflare_api_token)
            .json(&body)
            .send()
            .await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err("Failed to update IP".into())
        }
    }

    /// Lists all DNS records for the configured zone.
    ///
    /// # Returns
    /// - `Ok(Vec<RecordInfo>)` with all records if successful.
    /// - `Err` if the request fails or the response is invalid.
    pub async fn list_records(&self) -> Result<Vec<RecordInfo>, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let url = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records", self.config.cloudflare_zone_id);
        let resp = client
            .get(&url)
            .bearer_auth(&self.config.cloudflare_api_token)
            .send()
            .await?;
        let json: serde_json::Value = resp.json().await?;
        let mut records = Vec::new();
        if let Some(arr) = json["result"].as_array() {
            for rec in arr {
                let id = rec["id"].as_str().unwrap_or("").to_string();
                let name = rec["name"].as_str().unwrap_or("").to_string();
                let record_type = rec["type"].as_str().unwrap_or("").to_string();
                let content = rec["content"].as_str().unwrap_or("").to_string();
                records.push(RecordInfo { id, name, record_type, content });
            }
        }
        Ok(records)
    }
}

/// Simple struct to hold DNS record info.
#[derive(Debug, Clone)]
pub struct RecordInfo {
    pub id: String,
    pub name: String,
    pub record_type: String,
    pub content: String,
}
