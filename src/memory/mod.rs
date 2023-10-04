use std::{cell::RefCell, collections::HashMap};

use wasm_bindgen::JsValue;

// this is one way to persist data between ticks within Rust's memory, as opposed to
// keeping state in memory on game objects - but will be lost on global resets!
thread_local! {
  pub static COLONY_LIST: RefCell<Vec<String>> = RefCell::new(Vec::new());
}
