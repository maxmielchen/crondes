use std::env;

/// Configuration for the Cloudflare DNS update tool.
///
/// This struct holds all required environment variables for updating a Cloudflare DNS record.
///
/// Fields:
/// - `cloudflare_api_token`: The API token for authenticating with the Cloudflare API (env: `CF_API_TOKEN`).
/// - `cloudflare_zone_id`: The Cloudflare Zone ID where the DNS record resides (env: `CF_ZONE_ID`).
/// - `cloudflare_record_id`: The specific DNS record ID to update (env: `CF_RECORD_ID`).
/// - `update_interval_secs`: The interval in seconds between update attempts (env: `UPDATE_INTERVAL_SECS`).
#[derive(Debug)]
pub struct Config {
    pub cloudflare_api_token: String,
    pub cloudflare_zone_id: String,
    pub cloudflare_record_id: String,
    pub update_interval_secs: u64,
}

impl Config {
    /// Loads all required configuration from environment variables.
    ///
    /// # Errors
    /// Returns an error if any required environment variable is missing or invalid.
    pub fn from_env() -> Result<Self, String> {
        let cloudflare_api_token = env::var("CF_API_TOKEN").map_err(|_| "CF_API_TOKEN is missing".to_string())?;
        let cloudflare_zone_id = env::var("CF_ZONE_ID").map_err(|_| "CF_ZONE_ID is missing".to_string())?;
        let cloudflare_record_id = env::var("CF_RECORD_ID").map_err(|_| "CF_RECORD_ID is missing".to_string())?;
        let update_interval_secs = env::var("UPDATE_INTERVAL_SECS")
            .map_err(|_| "UPDATE_INTERVAL_SECS is missing".to_string())?
            .parse::<u64>()
            .map_err(|_| "UPDATE_INTERVAL_SECS must be a number".to_string())?;
        Ok(Config {
            cloudflare_api_token,
            cloudflare_zone_id,
            cloudflare_record_id,
            update_interval_secs,
        })
    }
}
