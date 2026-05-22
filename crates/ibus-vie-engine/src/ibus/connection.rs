use crate::error::EngineError;
use tracing::info;
use zbus::connection::Builder;
use zbus::Connection;

/// Connect to the IBus bus.
///
/// IBus daemon uses a private bus (not the session bus). When ibus-daemon
/// spawns an engine, it sets the `IBUS_ADDRESS` environment variable.
/// We can also discover the address from `~/.cache/ibus/bus/` socket files
/// or by calling `ibus address`.
pub async fn connect() -> Result<Connection, EngineError> {
    let address = ibus_address()?;
    info!("connecting to IBus bus at: {}", address);

    let connection = Builder::address(address.as_str())?
        .auth_mechanism(zbus::AuthMechanism::External)
        .build()
        .await?;

    info!("IBus bus connection established");
    Ok(connection)
}

/// Discover the IBus bus address.
///
/// Priority:
/// 1. `IBUS_ADDRESS` environment variable (set by ibus-daemon when spawning engines)
/// 2. Read from the IBus socket file in `~/.cache/ibus/bus/`
/// 3. Run `ibus address` as fallback
fn ibus_address() -> Result<String, EngineError> {
    // Check env first (set when ibus-daemon spawns us)
    if let Ok(addr) = std::env::var("IBUS_ADDRESS") {
        return Ok(addr);
    }

    // Fallback: read from IBus bus file
    if let Ok(addr) = read_bus_file() {
        return Ok(addr);
    }

    // Last resort: call `ibus address`
    let output = std::process::Command::new("ibus")
        .arg("address")
        .output()
        .map_err(|e| EngineError::Connection(format!("failed to run `ibus address`: {}", e)))?;

    if output.status.success() {
        let addr = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !addr.is_empty() {
            return Ok(addr);
        }
    }

    Err(EngineError::Connection(
        "cannot determine IBus bus address: IBUS_ADDRESS not set, bus file not found, `ibus address` failed".to_string(),
    ))
}

fn read_bus_file() -> Result<String, EngineError> {
    let machine_id = std::fs::read_to_string("/etc/machine-id")
        .or_else(|_| std::fs::read_to_string("/var/lib/dbus/machine-id"))
        .map_err(|e| EngineError::Connection(format!("cannot read machine-id: {}", e)))?;
    let machine_id = machine_id.trim();

    let display = std::env::var("DISPLAY").unwrap_or_else(|_| ":0".to_string());
    let display_id = display.trim_start_matches(':').replace('.', "-");

    let bus_dir = dirs_path();
    let bus_file = format!("{}/{}-{}", bus_dir, machine_id, display_id);

    let content = std::fs::read_to_string(&bus_file)
        .map_err(|e| EngineError::Connection(format!("cannot read IBus bus file {}: {}", bus_file, e)))?;

    for line in content.lines() {
        if let Some(addr) = line.strip_prefix("IBUS_ADDRESS=") {
            return Ok(addr.to_string());
        }
    }

    Err(EngineError::Connection(format!(
        "IBUS_ADDRESS not found in {}",
        bus_file
    )))
}

fn dirs_path() -> String {
    if let Ok(dir) = std::env::var("XDG_CACHE_HOME") {
        format!("{}/ibus/bus", dir)
    } else if let Ok(home) = std::env::var("HOME") {
        format!("{}/.cache/ibus/bus", home)
    } else {
        "/tmp/ibus/bus".to_string()
    }
}
