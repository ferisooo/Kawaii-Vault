//! Panic remote-wipe poller.
//!
//! Watches a user-configured Telegram bot for a trigger phrase. When the phrase
//! arrives, it securely destroys every vault on this machine and (per the
//! configured action) scrubs free space and reboots/shuts down.
//!
//! HONEST THREAT MODEL — do not oversell this:
//!   • It only acts while Kawaii Vault is RUNNING (i.e. on or after login). A
//!     powered-off PC that is kept offline, booted from a USB stick, or has its
//!     drive pulled will NEVER see the trigger, so this cannot defend against an
//!     attacker with physical possession of a powered-off machine.
//!   • The real at-rest protection when a machine is seized is the vault's
//!     encryption (the attacker gets ciphertext). This feature is a PROACTIVE
//!     "burn it now" switch for when the machine is still yours and online.
//!   • The trigger config (bot token + phrase) is stored in a plaintext file in
//!     the app data dir, because the poller must read it before any vault is
//!     unlocked. Leaking that file only lets someone TRIGGER a wipe of data they
//!     already cannot read — it exposes no vault contents.
//!
//! A queued message sent while the PC was off fires on the FIRST successful poll
//! after startup: Telegram holds updates until they are confirmed, and the first
//! getUpdates call (no offset) returns everything pending.

use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tauri::Manager;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

fn default_channel() -> String {
    "telegram".to_string()
}
fn default_action() -> String {
    "wipe_scrub_restart".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteWipeConfig {
    pub enabled: bool,
    #[serde(default = "default_channel")]
    pub channel: String,
    pub bot_token: String,
    pub trigger_phrase: String,
    /// One of: "wipe_scrub_restart", "wipe_restart", "wipe_shutdown", "wipe_only".
    #[serde(default = "default_action")]
    pub action: String,
    #[serde(default)]
    pub autostart: bool,
}

impl Default for RemoteWipeConfig {
    fn default() -> Self {
        RemoteWipeConfig {
            enabled: false,
            channel: default_channel(),
            bot_token: String::new(),
            trigger_phrase: String::new(),
            action: default_action(),
            autostart: false,
        }
    }
}

/// Path to the trigger config (plaintext JSON — see module doc for why).
pub fn config_path() -> PathBuf {
    crate::vault::app_data_dir().join(".remotewipe")
}

pub fn load_config() -> RemoteWipeConfig {
    match fs::read(config_path()) {
        Ok(bytes) => serde_json::from_slice(&bytes).unwrap_or_default(),
        Err(_) => RemoteWipeConfig::default(),
    }
}

pub fn save_config(cfg: &RemoteWipeConfig) -> Result<(), String> {
    let json = serde_json::to_vec_pretty(cfg).map_err(|e| format!("Serialize config: {}", e))?;
    let dir = crate::vault::app_data_dir();
    let _ = fs::create_dir_all(&dir);
    fs::write(config_path(), json).map_err(|e| format!("Write config: {}", e))
}

// ── Auto-start on login ──

#[cfg(target_os = "windows")]
const RUN_KEY: &str = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run";
#[cfg(target_os = "windows")]
const RUN_VALUE: &str = "KawaiiVault";

/// Register or unregister launch-on-login so a trigger that arrived while the
/// PC was off is caught on the next boot. Best-effort; never panics.
pub fn set_autostart(enabled: bool) -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| format!("current_exe: {}", e))?;

    #[cfg(target_os = "windows")]
    {
        let exe_str = exe.to_string_lossy().to_string();
        let status = if enabled {
            std::process::Command::new("reg")
                .args([
                    "add", RUN_KEY, "/v", RUN_VALUE, "/t", "REG_SZ", "/d", &exe_str, "/f",
                ])
                .creation_flags(CREATE_NO_WINDOW)
                .status()
        } else {
            std::process::Command::new("reg")
                .args(["delete", RUN_KEY, "/v", RUN_VALUE, "/f"])
                .creation_flags(CREATE_NO_WINDOW)
                .status()
        };
        // A "delete" of an absent value returns non-zero — that's fine.
        let _ = status;
        return Ok(());
    }

    #[cfg(target_os = "linux")]
    {
        let autostart_dir = std::env::var_os("HOME")
            .map(PathBuf::from)
            .map(|h| h.join(".config").join("autostart"))
            .ok_or("No HOME directory")?;
        let desktop = autostart_dir.join("kawaii-vault.desktop");
        if enabled {
            fs::create_dir_all(&autostart_dir).map_err(|e| format!("autostart dir: {}", e))?;
            let contents = format!(
                "[Desktop Entry]\nType=Application\nName=Kawaii Vault\nExec={}\nX-GNOME-Autostart-enabled=true\nHidden=false\n",
                exe.to_string_lossy()
            );
            fs::write(&desktop, contents).map_err(|e| format!("write .desktop: {}", e))?;
        } else {
            let _ = fs::remove_file(&desktop);
        }
        return Ok(());
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        // macOS / other: best-effort no-op.
        let _ = (exe, enabled);
        Ok(())
    }
}

// ── Telegram channel ──

/// Verify a bot token by calling getMe; returns the bot's "@username".
pub async fn verify_token(bot_token: &str) -> Result<String, String> {
    let token = bot_token.trim();
    if token.is_empty() {
        return Err("Enter a bot token".into());
    }
    let url = format!("https://api.telegram.org/bot{}/getMe", token);
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| format!("HTTP client: {}", e))?;
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Could not reach Telegram: {}", e))?;
    let json: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| format!("Bad response from Telegram: {}", e))?;
    if json.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        let desc = json
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("token rejected");
        return Err(format!("Telegram: {}", desc));
    }
    let username = json
        .get("result")
        .and_then(|r| r.get("username"))
        .and_then(|u| u.as_str())
        .unwrap_or("unknown");
    Ok(format!("@{}", username))
}

/// One long-poll getUpdates call. Returns the raw `result` array of updates.
async fn poll_updates(bot_token: &str, offset: Option<i64>) -> Result<Vec<serde_json::Value>, String> {
    let mut url = format!(
        "https://api.telegram.org/bot{}/getUpdates?timeout=25",
        bot_token
    );
    if let Some(o) = offset {
        url.push_str(&format!("&offset={}", o));
    }
    // Request timeout must exceed the server-side long-poll window.
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(35))
        .build()
        .map_err(|e| format!("HTTP client: {}", e))?;
    let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
    let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    if json.get("ok").and_then(|v| v.as_bool()) != Some(true) {
        return Err("Telegram getUpdates not ok".into());
    }
    Ok(json
        .get("result")
        .and_then(|r| r.as_array())
        .cloned()
        .unwrap_or_default())
}

/// Extract the text of a message/channel_post/edited variants from an update.
fn update_text(update: &serde_json::Value) -> Option<&str> {
    for key in ["message", "edited_message", "channel_post", "edited_channel_post"] {
        if let Some(t) = update.get(key).and_then(|m| m.get("text")).and_then(|t| t.as_str()) {
            return Some(t);
        }
    }
    None
}

/// Spawn the background poller. Runs on its own OS thread (so it never touches
/// the UI thread) and drives async HTTP via the Tauri runtime's block_on. Runs
/// regardless of unlock state.
pub fn start_poller(app: tauri::AppHandle) {
    std::thread::spawn(move || {
        let mut offset: Option<i64> = None;
        loop {
            let cfg = load_config();
            if !cfg.enabled
                || cfg.channel != "telegram"
                || cfg.bot_token.trim().is_empty()
                || cfg.trigger_phrase.trim().is_empty()
            {
                std::thread::sleep(Duration::from_secs(5));
                continue;
            }

            let token = cfg.bot_token.trim().to_string();
            let needle = cfg.trigger_phrase.trim().to_lowercase();

            match tauri::async_runtime::block_on(poll_updates(&token, offset)) {
                Ok(updates) => {
                    let mut max_id: Option<i64> = None;
                    for update in &updates {
                        if let Some(id) = update.get("update_id").and_then(|v| v.as_i64()) {
                            max_id = Some(max_id.map_or(id, |m| m.max(id)));
                        }
                        if let Some(text) = update_text(update) {
                            if text.to_lowercase().contains(&needle) {
                                // Trigger matched — burn everything and stop polling.
                                execute_wipe(&app, &cfg.action);
                                return;
                            }
                        }
                    }
                    // Confirm processed updates so they don't repeat next poll.
                    if let Some(m) = max_id {
                        offset = Some(m + 1);
                    }
                    // No sleep needed: getUpdates already blocked server-side for
                    // up to 25s. On an empty result just loop straight into the
                    // next long-poll.
                }
                Err(_) => {
                    // Network/parse hiccup — back off briefly, then retry.
                    std::thread::sleep(Duration::from_secs(5));
                }
            }
        }
    });
}

/// Execute the configured destructive action. Never panics.
fn execute_wipe(app: &tauri::AppHandle, action: &str) {
    // 1) Securely destroy every vault + clear the data directory.
    if let Ok(mut vm) = app.state::<crate::AppState>().vault_manager.lock() {
        let _ = vm.remote_wipe_all_vaults();
    }

    // 2) Optionally scrub free space (fills unallocated blocks with random).
    let dir = crate::vault::app_data_dir();
    if action.contains("scrub") {
        crate::vault::scrub_free_space(&dir);
    }

    // 3) Power action.
    match action {
        "wipe_only" => {}
        "wipe_shutdown" => power_off(),
        _ => reboot(), // wipe_restart / wipe_scrub_restart / anything else
    }
}

fn reboot() {
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("shutdown")
            .args(["/r", "/t", "0"])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn();
    }
    #[cfg(not(target_os = "windows"))]
    {
        if std::process::Command::new("shutdown").args(["-r", "now"]).spawn().is_err() {
            let _ = std::process::Command::new("systemctl").arg("reboot").spawn();
        }
    }
}

fn power_off() {
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("shutdown")
            .args(["/s", "/t", "0"])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn();
    }
    #[cfg(not(target_os = "windows"))]
    {
        if std::process::Command::new("shutdown").args(["-h", "now"]).spawn().is_err() {
            let _ = std::process::Command::new("systemctl").arg("poweroff").spawn();
        }
    }
}
