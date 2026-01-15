use leptos::logging;
use serde::{Serialize, de::DeserializeOwned};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "event"])]
    async fn listen(event: &str, handler: &Closure<dyn FnMut(JsValue)>) -> JsValue;
}

pub async fn tauri_invoke<T: DeserializeOwned>(
    cmd: &str,
    args: impl Serialize,
) -> Result<T, JsValue> {
    let args = to_value(&args).map_err(|e| JsValue::from_str(&e.to_string()))?;
    match invoke(cmd, args).await {
        Ok(result) => from_value::<T>(result).map_err(|e| JsValue::from_str(&e.to_string())),
        Err(e) => {
            logging::debug_error!("Cmd: {:#?}", cmd);
            logging::debug_error!("Error: {:#?}", e);
            Err(e)
        }
    }
}
