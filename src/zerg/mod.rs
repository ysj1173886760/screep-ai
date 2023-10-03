use log::warn;
use wasm_bindgen::JsValue;

use crate::error::SwarmError;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreepMemory {
    pub overlord: String,
    pub role: String,
    // pub colony: String,
}

impl CreepMemory {
    pub fn from_value(js_value: JsValue) -> Self {
        let result: Result<CreepMemory, _> = serde_wasm_bindgen::from_value(js_value);
        if result.is_err() {
            warn!("Parse creep memory failed.");
        }

        result.unwrap()
    }

    pub fn into_value(&self) -> JsValue {
        serde_wasm_bindgen::to_value(self).unwrap()
    }
}
