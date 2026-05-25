use ibus_vie_im::Action;
use tracing::debug;
use zbus::object_server::SignalEmitter;

use super::IbusEngineImpl;
use super::text::{ibus_text_value, ibus_text_with_underline};

const KEYVAL_BACKSPACE: u32 = 0xFF08;

pub async fn handle_preedit(
    engine: &mut IbusEngineImpl,
    emitter: &SignalEmitter<'_>,
    action: Action,
) -> bool {
    match action {
        Action::Update => {
            let preedit = engine.engine().preedit().to_string();
            debug!(preedit, "update [preedit]");
            let cursor_pos = preedit.chars().count() as u32;
            let _ = IbusEngineImpl::update_preedit_text(
                emitter,
                ibus_text_with_underline(&preedit),
                cursor_pos,
                true,
                0,
            )
            .await;
            true
        }
        Action::Commit(text) => {
            debug!(text, "commit [preedit]");
            let _ = IbusEngineImpl::commit_text(emitter, ibus_text_value(&text)).await;
            let _ = IbusEngineImpl::hide_preedit_text(emitter).await;
            engine.engine_mut().reset();
            true
        }
        Action::PassThrough => false,
    }
}

pub async fn handle_forward_key(
    engine: &mut IbusEngineImpl,
    emitter: &SignalEmitter<'_>,
    action: Action,
) -> bool {
    match action {
        Action::Update => {
            let new_preedit = engine.engine().preedit().to_string();
            debug!(preedit = new_preedit, "update [forward]");
            forward_replace(emitter, engine.forwarded_len, &new_preedit).await;
            engine.forwarded_len = new_preedit.chars().count();
            true
        }
        Action::Commit(text) => {
            debug!(text, "commit [forward]");
            for _ in 0..engine.forwarded_len {
                let _ = IbusEngineImpl::forward_key_event(emitter, KEYVAL_BACKSPACE, 0, 0).await;
            }
            engine.forwarded_len = 0;
            let _ = IbusEngineImpl::commit_text(emitter, ibus_text_value(&text)).await;
            engine.engine_mut().reset();
            true
        }
        Action::PassThrough => false,
    }
}

/// Erase `old_len` forwarded chars then forward all chars of `new_text`.
async fn forward_replace(emitter: &SignalEmitter<'_>, old_len: usize, new_text: &str) {
    for _ in 0..old_len {
        let _ = IbusEngineImpl::forward_key_event(emitter, KEYVAL_BACKSPACE, 0, 0).await;
    }
    for c in new_text.chars() {
        let _ = IbusEngineImpl::forward_key_event(emitter, char_to_keyval(c), 0, 0).await;
    }
}

fn char_to_keyval(c: char) -> u32 {
    let cp = c as u32;
    if cp <= 0xFF { cp } else { 0x01000000 | cp }
}

pub async fn commit_pending_preedit(engine: &mut IbusEngineImpl, emitter: &SignalEmitter<'_>) {
    if !engine.engine().preedit().is_empty() {
        let preedit = engine.engine().preedit().to_string();
        let _ = IbusEngineImpl::commit_text(emitter, ibus_text_value(&preedit)).await;
        let _ = IbusEngineImpl::hide_preedit_text(emitter).await;
    }
}

pub async fn hide_preedit(emitter: &SignalEmitter<'_>) {
    let _ = IbusEngineImpl::hide_preedit_text(emitter).await;
}
