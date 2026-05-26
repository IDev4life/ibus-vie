use std::sync::OnceLock;
use tracing::debug;

const REPO: &str = "IDev4life/ibus-vie";

static LATEST: OnceLock<Option<String>> = OnceLock::new();

pub fn current() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Label shown in IBus property menu.
pub fn version_label() -> String {
    match LATEST.get() {
        Some(Some(latest)) if latest.as_str() != current() => {
            format!("v{} (v{} available)", current(), latest)
        }
        _ => format!("v{}", current()),
    }
}

/// Spawn a one-shot background thread to check GitHub for the latest release.
/// Subsequent calls are no-ops (OnceLock already set).
pub fn spawn_check() {
    if LATEST.get().is_some() {
        return;
    }
    std::thread::spawn(|| {
        let result = fetch_latest();
        let _ = LATEST.set(result);
        debug!("update check complete: {:?}", LATEST.get());
    });
}

fn fetch_latest() -> Option<String> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", REPO);
    let output = std::process::Command::new("curl")
        .args(["-sf", "--max-time", "5", "-H", "User-Agent: ibus-vie", &url])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    extract_tag(&String::from_utf8(output.stdout).ok()?)
}

fn extract_tag(json: &str) -> Option<String> {
    let key = "\"tag_name\":\"";
    let start = json.find(key)? + key.len();
    let end = start + json[start..].find('"')?;
    let tag = &json[start..end];
    let version = tag
        .trim_start_matches("ibus-vie-")
        .trim_start_matches('v');
    Some(version.to_string())
}
