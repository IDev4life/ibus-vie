use ibus_vie_im::{Engine, KeyEvent, TelexEngine, VniEngine};
use tracing::{debug, info};
use zbus::object_server::SignalEmitter;
use zbus::zvariant::Value;

use super::factory;
use super::props;
use super::signals;

/// IBus Engine interface implementation.
///
/// Uses hybrid mode:
/// - Apps supporting surrounding text: DeleteSurroundingText + CommitText (no underline)
/// - Apps without surrounding text (terminals): UpdatePreeditText + CommitText (has underline)
pub struct IbusEngineImpl {
    engine: Box<dyn Engine + Send + Sync>,
    method: String,
    prev_committed_chars: usize,
    has_surrounding_text: bool,
}

impl IbusEngineImpl {
    pub fn new(method: String) -> Self {
        let engine: Box<dyn Engine + Send + Sync> = match method.as_str() {
            "vni" => Box::new(VniEngine::new()),
            _ => Box::new(TelexEngine::new()),
        };
        info!("engine created with method: {}", method);
        Self {
            engine,
            method,
            prev_committed_chars: 0,
            has_surrounding_text: false,
        }
    }

    // --- Accessors for signals module ---

    pub fn engine(&self) -> &dyn Engine {
        &*self.engine
    }

    pub fn engine_mut(&mut self) -> &mut dyn Engine {
        &mut *self.engine
    }

    pub fn prev_committed_chars(&self) -> usize {
        self.prev_committed_chars
    }

    pub fn set_prev_committed_chars(&mut self, n: usize) {
        self.prev_committed_chars = n;
    }
}

/// DBus interface: org.freedesktop.IBus.Engine
#[zbus::interface(name = "org.freedesktop.IBus.Engine")]
impl IbusEngineImpl {
    async fn process_key_event(
        &mut self,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
        keyval: u32,
        _keycode: u32,
        state: u32,
    ) -> bool {
        let is_release = (state & (1 << 30)) != 0;
        if is_release {
            return false;
        }

        let has_ctrl = (state & (1 << 2)) != 0;
        let has_alt = (state & (1 << 3)) != 0;
        if has_ctrl || has_alt {
            if self.has_surrounding_text {
                self.prev_committed_chars = 0;
            } else if !self.engine.preedit().is_empty() {
                signals::commit_pending_preedit(self, &emitter).await;
            }
            self.engine.reset();
            return false;
        }

        let ev = match keyval {
            0xff08 => KeyEvent::backspace(),
            0xff1b => {
                if !self.has_surrounding_text {
                    signals::hide_preedit(&emitter).await;
                }
                self.prev_committed_chars = 0;
                self.engine.reset();
                return false;
            }
            0xff0d => {
                if !self.has_surrounding_text && !self.engine.preedit().is_empty() {
                    signals::commit_pending_preedit(self, &emitter).await;
                }
                self.prev_committed_chars = 0;
                self.engine.reset();
                return false;
            }
            kv if (0x20..=0x7e).contains(&kv) => KeyEvent::from_char(kv as u8 as char),
            _ => return false,
        };

        let action = self.engine.key(ev);

        if self.has_surrounding_text {
            signals::handle_surrounding_text(self, &emitter, action).await
        } else {
            signals::handle_preedit(self, &emitter, action).await
        }
    }

    fn set_surrounding_text(&mut self, _text: Value<'_>, _cursor_index: u32, _anchor_pos: u32) {
        if !self.has_surrounding_text {
            debug!("surrounding text support detected");
            self.has_surrounding_text = true;
        }
    }

    fn focus_in(&mut self) {
        self.has_surrounding_text = false;
        self.prev_committed_chars = 0;
        debug!(method = %self.method, "focus in");
    }

    async fn focus_out(&mut self, #[zbus(signal_emitter)] emitter: SignalEmitter<'_>) {
        if !self.has_surrounding_text && !self.engine.preedit().is_empty() {
            signals::commit_pending_preedit(self, &emitter).await;
        }
        self.prev_committed_chars = 0;
        self.engine.reset();
        debug!("focus out");
    }

    async fn reset(&mut self, #[zbus(signal_emitter)] emitter: SignalEmitter<'_>) {
        if !self.has_surrounding_text && !self.engine.preedit().is_empty() {
            signals::hide_preedit(&emitter).await;
        }
        self.prev_committed_chars = 0;
        self.engine.reset();
        debug!("engine reset");
    }

    async fn enable(&mut self, #[zbus(signal_emitter)] emitter: SignalEmitter<'_>) {
        let prop_list = props::method_prop_list(&self.method);
        match Self::register_properties(&emitter, prop_list).await {
            Ok(()) => debug!(method = %self.method, "engine enabled, properties registered"),
            Err(e) => tracing::error!("register_properties failed: {}", e),
        }
    }

    fn disable(&mut self) {
        self.prev_committed_chars = 0;
        self.engine.reset();
        debug!(method = %self.method, "engine disabled");
    }

    fn set_cursor_location(&self, _x: i32, _y: i32, _w: i32, _h: i32) {}

    fn set_capabilities(&self, _caps: u32) {}

    fn panel_extension_register_keys(&self, _data: Value<'_>) {}

    async fn property_activate(
        &mut self,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
        prop_name: &str,
        prop_state: u32,
    ) {
        // Only act on CHECKED state (1). IBus sends activate for both the
        // newly-checked and the newly-unchecked radio items.
        if prop_state != 1 {
            return;
        }

        let new_method = match prop_name {
            "method-telex" => "telex",
            "method-vni" => "vni",
            _ => return,
        };

        if new_method == self.method {
            return;
        }

        // Commit any pending preedit before switching
        if !self.engine.preedit().is_empty() {
            if self.has_surrounding_text {
                let preedit = self.engine.preedit().to_string();
                let prev = self.prev_committed_chars;
                if prev > 0 {
                    let _ = Self::delete_surrounding_text(&emitter, -(prev as i32), prev as u32)
                        .await;
                }
                let _ =
                    Self::commit_text(&emitter, super::ibus_text::ibus_text_value(&preedit)).await;
            } else {
                signals::commit_pending_preedit(self, &emitter).await;
            }
        }

        // Switch method and persist to shared state
        self.method = new_method.to_string();
        self.engine = match new_method {
            "vni" => Box::new(VniEngine::new()),
            _ => Box::new(TelexEngine::new()),
        };
        self.prev_committed_chars = 0;
        factory::set_active_method(new_method);

        // Update the property menu to reflect new selection
        let updated_prop = props::method_menu_property(new_method);
        let _ = Self::update_property(&emitter, updated_prop).await;

        info!("switched method to {}", new_method);
    }

    fn destroy(&mut self) {
        debug!("engine destroyed");
    }

    // --- Signals ---

    #[zbus(signal)]
    pub async fn commit_text(emitter: &SignalEmitter<'_>, text: Value<'_>) -> zbus::Result<()>;

    #[zbus(signal)]
    pub async fn update_preedit_text(
        emitter: &SignalEmitter<'_>,
        text: Value<'_>,
        cursor_pos: u32,
        visible: bool,
        mode: u32,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    pub async fn hide_preedit_text(emitter: &SignalEmitter<'_>) -> zbus::Result<()>;

    #[zbus(signal)]
    pub async fn delete_surrounding_text(
        emitter: &SignalEmitter<'_>,
        offset: i32,
        nchars: u32,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    pub async fn register_properties(
        emitter: &SignalEmitter<'_>,
        props: Value<'_>,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    pub async fn update_property(
        emitter: &SignalEmitter<'_>,
        prop: Value<'_>,
    ) -> zbus::Result<()>;
}
