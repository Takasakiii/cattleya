#![allow(clippy::unused_unit)]

use std::sync::Arc;

use cattleya::request::{VioletLogData, VioletRequest};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Cattleya {
    client: Arc<VioletRequest>,
}

#[wasm_bindgen]
impl Cattleya {
    #[wasm_bindgen(constructor)]
    pub fn new(base_url: String, token: String) -> Result<Cattleya, JsValue> {
        console_error_panic_hook::set_once();
        let client =
            VioletRequest::new(&token, base_url).map_err(|e| JsValue::from_str(&e.to_string()))?;

        Ok(Cattleya {
            client: Arc::new(client),
        })
    }

    pub fn custom_error(&self, err_level: String, message: String, stack_trace: String) -> Promise {
        let log_data = VioletLogData {
            error_level: err_level,
            message,
            stack_trace,
        };

        let client = self.client.clone();

        future_to_promise(async move {
            client
                .send_log(log_data)
                .await
                .map_err(|e| JsValue::from_str(&e.to_string()))?;

            Ok(JsValue::NULL)
        })
    }
}
