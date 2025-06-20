mod config;
mod cloudflare;
mod ip;

use std::error::Error;
use cloudflare::Cloudflare;
use log::{info, error};
use std::sync::Arc;
use tokio::sync::Notify;
use std::time::Duration;


/// Checks all required credentials and IDs (API token, zone ID, record ID).
/// If the record ID is invalid, logs all available records and returns an error.
pub async fn check_all_info(cf: &Cloudflare) -> Result<(), Box<dyn Error>> {
    if !cf.api_token_right().await? {
        return Err("API token is invalid".into());
    }
    if !cf.zone_id_right().await? {
        return Err("Zone ID is invalid".into());
    }
    if !cf.record_id_right().await? {
        error!("Record ID is invalid. Listing all available records:");
        let records = cf.list_records().await?;
        for rec in records {
            error!("ID: {} | Name: {} | Type: {} | Content: {}", rec.id, rec.name, rec.record_type, rec.content);
        }
        return Err("Record ID is invalid".into());
    }
    Ok(())
}

/// Initializes the config from environment variables and logs the values.
pub fn init_and_log_config() -> Result<config::Config, Box<dyn Error>> {
    let cfg = config::Config::from_env()?;
    info!("Loaded config:");
    info!("  CF_API_TOKEN: {}", &cfg.cloudflare_api_token);
    info!("  CF_ZONE_ID: {}", &cfg.cloudflare_zone_id);
    info!("  CF_RECORD_ID: {}", &cfg.cloudflare_record_id);
    info!("  CF_RECORD_NAME: {}", &cfg.cloudflare_record_name);
    info!("  UPDATE_INTERVAL_SECS: {}", cfg.update_interval_secs);
    Ok(cfg)
}

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Logger initialized");

    // 1. Config laden
    let cfg = match init_and_log_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Config error: {}", e);
            return;
        }
    };
    // 2. Cloudflare-Objekt erstellen
    let cf = Cloudflare::new(cfg);

    // 3. Scheduler starten
    let shutdown = Arc::new(Notify::new());
    let shutdown_signal = shutdown.clone();
    let interval = Duration::from_secs(cf.config.update_interval_secs);

    tokio::spawn(async move {
        let mut run_count = 0;
        loop {
            run_count += 1;
            info!("--- Update loop iteration #{} ---", run_count);
            info!("Starting update cycle...");
            if let Err(e) = update(&cf).await {
                error!("Update failed: {}. Shutting down scheduler.", e);
                shutdown_signal.notify_waiters();
                break;
            } else {
                info!("Update completed successfully.");
            }
            info!("Waiting {} seconds until next iteration...", interval.as_secs());
            tokio::select! {
                _ = tokio::time::sleep(interval) => {},
                _ = shutdown_signal.notified() => break,
            }
        }
    });

    // Warten auf Shutdown (z.B. durch Fehler oder externes Signal)
    shutdown.notified().await;
    info!("Scheduler stopped. Exiting.");
}

/// Führt einen vollständigen Update-Zyklus durch: check_all_info und ggf. IP-Update.
async fn update(cf: &Cloudflare) -> Result<(), Box<dyn Error>> {
    info!("Checking Cloudflare credentials and IDs...");
    check_all_info(cf).await?;
    let current_dns_ip = cf.current_ip().await?;
    info!("Current DNS IP: {}", current_dns_ip);
    let public_ip = crate::ip::fetch_public_ip().await?;
    info!("Public IP: {}", public_ip);
    if current_dns_ip != public_ip {
        info!("Updating DNS record: {} → {}", current_dns_ip, public_ip);
        match cf.update_ip(&public_ip).await {
            Ok(response_body) => info!("DNS record updated successfully. Response: {}", response_body),
            Err(e) => {
                error!("Error updating DNS record: {}", e);
                return Err(e);
            }
        }
    } else {
        info!("No update needed. Public IP unchanged: {}", public_ip);
    }
    Ok(())
}