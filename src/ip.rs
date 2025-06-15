use std::error::Error;
use std::net::IpAddr;

/// List of external services to fetch the public IP address from.
const IP_SERVICES: &[&str] = [
    "https://api.ipify.org",
    "https://ifconfig.me/ip",
    "https://checkip.amazonaws.com",
    "https://ipecho.net/plain",
    "https://ident.me",
];

/// Attempts to fetch the current public IPv4 address from multiple external services.
///
/// The function iterates through a list of known IP services and returns the first valid IPv4 address found.
/// Each response is strictly validated to ensure it is a valid IP address.
///
/// # Errors
/// Returns an error if no valid public IP address could be determined from any of the services.
pub async fn fetch_public_ip() -> Result<String, Box<dyn Error>> {
    for &url in IP_SERVICES {
        if let Ok(resp) = reqwest::get(url).await.and_then(|r| r.text()).await {
            let ip = resp.trim();
            if let Ok(parsed) = ip.parse::<IpAddr>() {
                if parsed.is_ipv4() {
                    return Ok(ip.to_string());
                }
            }
        }
    }
    Err("No valid public IP address could be determined".into())
}
