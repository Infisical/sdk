extern crate console_error_panic_hook;
use std::{rc::Rc, sync::RwLock};

use infisical_json::client::Client as JsonClient;
use js_sys::Promise;
use log::Level;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

fn convert_level(level: LogLevel) -> Level {
    match level {
        LogLevel::Trace => Level::Trace,
        LogLevel::Debug => Level::Debug,
        LogLevel::Info => Level::Info,
        LogLevel::Warn => Level::Warn,
        LogLevel::Error => Level::Error,
    }
}

// Rc<RwLock<...>> is to avoid needing to take ownership of the Client during our async run_command function
// https://github.com/rustwasm/wasm-bindgen/issues/2195#issuecomment-799588401
#[wasm_bindgen]
pub struct InfisicalClient(Rc<RwLock<JsonClient>>);

#[wasm_bindgen]
impl InfisicalClient {
    #[wasm_bindgen(constructor)]
    pub fn new(settings_input: Option<String>, log_level: Option<LogLevel>) -> Self {
        console_error_panic_hook::set_once();
        if let Err(e) =
            console_log::init_with_level(convert_level(log_level.unwrap_or(LogLevel::Info)))
        {
            panic!("failed to initialize logger: {:?}", e);
        }

        Self(Rc::new(RwLock::new(infisical_json::client::Client::new(
            settings_input,
        ))))
    }

    #[wasm_bindgen]
    pub fn run_command(&mut self, js_input: String) -> Promise {
        let rc = self.0.clone();
        // TODO: We should probably switch to an async-aware RwLock here,
        // but it probably doesn't matter much in a single threaded environment
        #[allow(clippy::await_holding_lock)]
        future_to_promise(async move {
            let mut client = rc.write().unwrap();
            let result = client.run_command(&js_input).await;
            Ok(result.into())
        })
    }
}
