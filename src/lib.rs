mod riichi;
extern crate wasm_bindgen;
extern crate serde;
extern crate serde_json;

use wasm_bindgen::prelude::*;
use riichi::hand::Hand;
use serde_json::json;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn get_hand_shanten(hand_string: &str) -> String {
    match Hand::from_text(hand_string) {
        Ok(mut hand) => {
            let shanten = hand.shanten();
            return json!({
                "success": {
                    "shanten": shanten
                }
            }).to_string();
        },
        Err(error) => {
            return json!({
                "error": {
                    "code": error.code,
                    "message": error.message
                }
            }).to_string();
        }
    }
}