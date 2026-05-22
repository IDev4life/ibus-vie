use ibus_vie_im::Action;
use tracing::debug;
use zbus::object_server::SignalEmitter;

use super::engine_impl::IbusEngineImpl;
use super::ibus_text::ibus_text_value;

/// Surrounding text mode: DeleteSurroundingText + CommitText (no underline).
///
/// Used when the client app supports surrounding text (detected via SetSurroundingText).
pub async fn handle_surrounding_text(
    engine: &mut IbusEngineImpl,
    emitter: &SignalEmitter<'_>,
    action: Action,
) -> bool {
    match action {
        Action::Update => {
            let preedit = engine.engine().preedit().to_string();
            let prev = engine.prev_committed_chars();
            debug!(preedit, prev, "update [surrounding]");

            if prev > 0 {
                let _ = IbusEngineImpl::delete_surrounding_text(
                    emitter,
                    -(prev as i32),
                    prev as u32,
                )
                .await;
            }

            let _ = IbusEngineImpl::commit_text(emitter, ibus_text_value(&preedit)).await;
            engine.set_prev_committed_chars(preedit.chars().count());
            true
        }
        Action::Commit(text) => {
            let prev = engine.prev_committed_chars();
            debug!(text, prev, "commit [surrounding]");

            if prev > 0 {
                let _ = IbusEngineImpl::delete_surrounding_text(
                    emitter,
                    -(prev as i32),
                    prev as u32,
                )
                .await;
            }

            let _ = IbusEngineImpl::commit_text(emitter, ibus_text_value(&text)).await;
            engine.set_prev_committed_chars(0);
            engine.engine_mut().reset();
            true
        }
        Action::PassThrough => {
            engine.set_prev_committed_chars(0);
            engine.engine_mut().reset();
            false
        }
    }
}

/// Preedit mode: UpdatePreeditText + CommitText (has underline, for terminals).
///
/// Used when the client app does not support surrounding text.
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
                ibus_text_value(&preedit),
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

/// Commit any pending preedit text (preedit mode only).
pub async fn commit_pending_preedit(engine: &mut IbusEngineImpl, emitter: &SignalEmitter<'_>) {
    if !engine.engine().preedit().is_empty() {
        let preedit = engine.engine().preedit().to_string();
        let _ = IbusEngineImpl::commit_text(emitter, ibus_text_value(&preedit)).await;
        let _ = IbusEngineImpl::hide_preedit_text(emitter).await;
    }
}

/// Hide preedit (preedit mode only).
pub async fn hide_preedit(emitter: &SignalEmitter<'_>) {
    let _ = IbusEngineImpl::hide_preedit_text(emitter).await;
}

// Re-export signal definitions — these live on IbusEngineImpl via #[zbus(signal)]
// and are called as IbusEngineImpl::signal_name(emitter, args...).await
// The actual signal declarations remain in engine_impl.rs because zbus requires them
// to be in the #[zbus::interface] impl block.
