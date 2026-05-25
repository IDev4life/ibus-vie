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

/// Forward-key mode: send delta key events instead of UpdatePreeditText.
///
/// `prev_preedit` is the engine preedit BEFORE the key was processed.
/// Uses a delta approach: only forward the minimal backspace+chars needed,
/// which avoids Chrome's "erase-all-then-retype" display lag.
pub async fn handle_forward_key(
    engine: &mut IbusEngineImpl,
    emitter: &SignalEmitter<'_>,
    action: Action,
    prev_preedit: &str,
) -> bool {
    match action {
        Action::Update => {
            let new_preedit = engine.engine().preedit().to_string();
            debug!(preedit = new_preedit, "update [forward]");
            forward_delta(emitter, prev_preedit, &new_preedit).await;
            true
        }
        Action::Commit(text) => {
            debug!(text, "commit [forward]");
            let prev_len = prev_preedit.chars().count();
            for _ in 0..prev_len {
                let _ = IbusEngineImpl::forward_key_event(emitter, KEYVAL_BACKSPACE, 0, 0).await;
            }
            let _ = IbusEngineImpl::commit_text(emitter, ibus_text_value(&text)).await;
            engine.engine_mut().reset();
            true
        }
        Action::PassThrough => false,
    }
}

/// Forward only the minimal diff between `old` and `new` preedit.
///
/// For simple appends (typing one more char), this sends exactly one
/// ForwardKeyEvent without any backspacing — Chrome and terminals see
/// each character appear immediately as typed.
async fn forward_delta(emitter: &SignalEmitter<'_>, old: &str, new: &str) {
    let common = old
        .chars()
        .zip(new.chars())
        .take_while(|(a, b)| a == b)
        .count();

    let backspace_count = old.chars().count() - common;
    for _ in 0..backspace_count {
        let _ = IbusEngineImpl::forward_key_event(emitter, KEYVAL_BACKSPACE, 0, 0).await;
    }
    for c in new.chars().skip(common) {
        let _ = IbusEngineImpl::forward_key_event(emitter, char_to_keyval(c), 0, 0).await;
    }
}

fn char_to_keyval(c: char) -> u32 {
    let cp = c as u32;
    // Latin-1 range fits directly; everything else uses the X Unicode keyval prefix.
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
