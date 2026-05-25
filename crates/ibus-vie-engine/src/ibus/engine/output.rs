use ibus_vie_im::Action;
use tracing::debug;
use zbus::object_server::SignalEmitter;

use super::IbusEngineImpl;
use super::text::{ibus_lookup_table, ibus_text_value, ibus_text_with_underline};

pub async fn handle_preedit(
    engine: &mut IbusEngineImpl,
    emitter: &SignalEmitter<'_>,
    action: Action,
) -> bool {
    match action {
        Action::Update => {
            let preedit = engine.engine().preedit().to_string();
            debug!(preedit, "update");
            if engine.is_popup_mode() {
                let _ = IbusEngineImpl::update_preedit_text(emitter, ibus_text_value(""), 0, false, 0).await;
                let _ = IbusEngineImpl::update_lookup_table(emitter, ibus_lookup_table(&preedit), true).await;
            } else {
                let cursor_pos = preedit.chars().count() as u32;
                let _ = IbusEngineImpl::update_preedit_text(
                    emitter,
                    ibus_text_with_underline(&preedit),
                    cursor_pos,
                    true,
                    0,
                )
                .await;
            }
            true
        }
        Action::Commit(text) => {
            debug!(text, "commit");
            let _ = IbusEngineImpl::commit_text(emitter, ibus_text_value(&text)).await;
            let _ = IbusEngineImpl::hide_preedit_text(emitter).await;
            if engine.is_popup_mode() {
                let _ = IbusEngineImpl::hide_lookup_table(emitter).await;
            }
            engine.engine_mut().reset();
            true
        }
        Action::PassThrough => false,
    }
}

pub async fn commit_pending_preedit(engine: &mut IbusEngineImpl, emitter: &SignalEmitter<'_>) {
    if !engine.engine().preedit().is_empty() {
        let preedit = engine.engine().preedit().to_string();
        let _ = IbusEngineImpl::commit_text(emitter, ibus_text_value(&preedit)).await;
        let _ = IbusEngineImpl::hide_preedit_text(emitter).await;
        if engine.is_popup_mode() {
            let _ = IbusEngineImpl::hide_lookup_table(emitter).await;
        }
    }
}

pub async fn hide_preedit(engine: &IbusEngineImpl, emitter: &SignalEmitter<'_>) {
    let _ = IbusEngineImpl::hide_preedit_text(emitter).await;
    if engine.is_popup_mode() {
        let _ = IbusEngineImpl::hide_lookup_table(emitter).await;
    }
}
