use super::CmdResult;
use crate::module::mihomo::MihomoManager;

#[tauri::command]
pub async fn get_proxies() -> CmdResult<serde_json::Value> {
    let mannager = MihomoManager::global();

    mannager
        .refresh_proxies()
        .await
        .map(|_| mannager.get_proxies())
        .or_else(|_| Ok(mannager.get_proxies()))
}

#[tauri::command]
pub async fn get_providers_proxies() -> CmdResult<serde_json::Value> {
    let mannager = MihomoManager::global();

    mannager
        .refresh_providers_proxies()
        .await
        .map(|_| mannager.get_providers_proxies())
        .or_else(|_| Ok(mannager.get_providers_proxies()))
}
