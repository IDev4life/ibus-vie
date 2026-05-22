use ibus_vie_im::{Action, Engine, KeyEvent, TelexEngine, ViqrEngine, VniEngine};
use tracing::{debug, info};

/// IBus Engine interface implementation.
///
/// This wraps our pure FSM engine and translates between IBus DBus protocol
/// and our internal Action/KeyEvent types.
pub struct IbusEngineImpl {
    /// The underlying input method engine.
    engine: Box<dyn Engine + Send + Sync>,
    /// Method name for logging.
    method: String,
}

impl IbusEngineImpl {
    pub fn new(method: String) -> Self {
        let engine: Box<dyn Engine + Send + Sync> = match method.as_str() {
            "vni" => Box::new(VniEngine::new()),
            "viqr" => Box::new(ViqrEngine::new()),
            _ => Box::new(TelexEngine::new()),
        };
        info!("engine created with method: {}", method);
        Self { engine, method }
    }
}

/// DBus interface implementation for org.freedesktop.IBus.Engine.
///
/// This is the core interface that ibus-daemon calls when keys are pressed.
#[zbus::interface(name = "org.freedesktop.IBus.Engine")]
impl IbusEngineImpl {
    /// Process a key event from ibus-daemon.
    ///
    /// Returns true if the key was consumed, false to pass through.
    fn process_key_event(&mut self, keyval: u32, _keycode: u32, state: u32) -> bool {
        // IBus key state: bit 30 = release, bit 0 = shift, bit 2 = ctrl, bit 3 = alt
        let is_release = (state & (1 << 30)) != 0;
        if is_release {
            return false;
        }

        // Ignore keys with Ctrl or Alt modifiers
        let has_ctrl = (state & (1 << 2)) != 0;
        let has_alt = (state & (1 << 3)) != 0;
        if has_ctrl || has_alt {
            // If there's preedit, commit it first
            if !self.engine.preedit().is_empty() {
                let _preedit = self.engine.preedit().to_string();
                self.engine.reset();
            }
            return false;
        }

        let ev = match keyval {
            0xff08 => KeyEvent::backspace(), // BackSpace
            0xff1b => KeyEvent::escape(),    // Escape
            0xff0d => {
                // Return/Enter: commit preedit
                if self.engine.preedit().is_empty() {
                    return false;
                }
                let committed = self.engine.preedit().to_string();
                self.engine.reset();
                debug!(committed, "commit on Enter");
                return true;
            }
            kv if (0x20..=0x7e).contains(&kv) => {
                // Printable ASCII
                KeyEvent::from_char(kv as u8 as char)
            }
            _ => return false,
        };

        let action = self.engine.key(ev);
        match action {
            Action::Update => {
                let preedit = self.engine.preedit().to_string();
                debug!(preedit, "update preedit");
                true
            }
            Action::Commit(text) => {
                debug!(text, "commit text");
                self.engine.reset();
                true
            }
            Action::PassThrough => false,
        }
    }

    /// Focus in: engine activated.
    fn focus_in(&mut self) {
        debug!(method = %self.method, "focus in");
    }

    /// Focus out: commit any pending preedit.
    fn focus_out(&mut self) {
        if !self.engine.preedit().is_empty() {
            let _committed = self.engine.preedit().to_string();
            self.engine.reset();
            debug!("focus out: committed pending preedit");
        }
    }

    /// Reset the engine state.
    fn reset(&mut self) {
        self.engine.reset();
        debug!("engine reset");
    }

    /// Enable the engine.
    fn enable(&mut self) {
        debug!(method = %self.method, "engine enabled");
    }

    /// Disable the engine.
    fn disable(&mut self) {
        self.engine.reset();
        debug!(method = %self.method, "engine disabled");
    }

    /// Set cursor location (we don't need this but IBus calls it).
    fn set_cursor_location(&self, _x: i32, _y: i32, _w: i32, _h: i32) {}

    /// Property activate (IBus panel properties).
    fn property_activate(&self, _prop_name: &str, _prop_state: u32) {}

    /// Destroy the engine.
    fn destroy(&mut self) {
        debug!("engine destroyed");
    }
}
