use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use js_sys::Error;

#[derive(Serialize, Deserialize)]
pub enum Dict {
    String(String),
    Int(i32),
    Float(f32),
    List(Vec<Option<Dict>>),
    Object(HashMap<String, Option<Dict>>)
}

type JsResult = Result<JsValue, Error>;

// no need to deserialize json bc of js but still gonna do it anyway
#[wasm_bindgen]
pub fn des_json(s: String) -> JsResult {
    let v: Dict = serde_json::from_str(&s)
        .map_err(|_| Error::new("Invalid JSON"))?;

    Ok(serde_wasm_bindgen::to_value(&v).unwrap())
}

#[wasm_bindgen]
pub fn des_toml(s: String) -> JsResult {

}

#[wasm_bindgen]
pub fn des_ron(s: String) -> JsResult {

}

#[wasm_bindgen]
pub fn des_yaml(s: String) -> JsResult {

}

#[wasm_bindgen]
pub fn des_pickle(s: String) -> JsResult {

}

#[wasm_bindgen]
pub fn des_qs(s: String) -> JsResult {

}

#[wasm_bindgen]
pub fn des_lexpr(s: String) -> JsResult {

}

#[wasm_bindgen]
pub fn convert_to_all(d: JsValue) -> JsResult {

}