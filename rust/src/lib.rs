use erros::CattleyaInitError;
use once_cell::sync::OnceCell;
use request::{VioletLogData, VioletRequest};

pub mod erros;
mod macros;
pub(crate) mod request;

static CATTLEYA_STATE: OnceCell<CattleyaState> = OnceCell::new();

pub(self) struct CattleyaState {
    violet_request: VioletRequest,
}

pub fn init_cattleya(base_url: String, token: &str) -> Result<(), CattleyaInitError> {
    let client = VioletRequest::new(token, base_url)?;
    CATTLEYA_STATE.set(CattleyaState {
        violet_request: client,
    })?;

    Ok(())
}

async fn emit_log_base(level: String, message: String, target: String) {
    let log_data = VioletLogData {
        error_level: level,
        message,
        stack_trace: target,
    };

    let state = CATTLEYA_STATE
        .get()
        .expect("Cattleya not initialized, use cattleya_rs::init_cattleya");

    state.violet_request.send_log(log_data).await.ok();
}

#[cfg(feature = "future")]
pub async fn emit_log(level: String, message: String, target: String) {
    emit_log_base(level, message, target).await;
}

#[cfg(feature = "tokio")]
pub fn emit_log(level: String, message: String, target: String) {
    tokio::spawn(async move { emit_log_base(level, message, target).await });
}

#[cfg(feature = "blocking")]
fn emit_log_blocking(level: String, message: String, target: String) {
    use futures::executor::block_on;

    block_on(async move { emit_log_base(level, message, target).await });
}

#[cfg(all(feature = "blocking", not(feature = "thread")))]
pub fn emit_log(level: String, message: String, target: String) {
    emit_log_blocking(level, message, target);
}

#[cfg(feature = "thread")]
pub fn emit_log(level: String, message: String, target: String) {
    use std::thread;

    thread::spawn(move || {
        emit_log_blocking(level, message, target);
    });
}
