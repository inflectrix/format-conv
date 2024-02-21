use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub enum Dict {
    String(String),
    Int(i32),
    Float(f32),
    List(Vec<Option<AnyData>>),
    Object(HashMap<String, Option<AnyData>>)
}

// no need to deserialize json bc of js but still gonna do it anyway
#[wasm_bindgen]
pub fn des_json(s: String) -> Dict {

}

#[wasm_bindgen]
pub fn des_toml(s: String) -> Dict {

}

#[wasm_bindgen]
pub fn des_ron(s: String) -> Dict {

}

#[wasm_bindgen]
pub fn des_yaml(s: String) -> Dict {

}

#[wasm_bindgen]
pub fn des_pickle(s: String) -> Dict {

}

#[wasm_bindgen]
pub fn des_qs(s: String) -> Dict {

}

#[wasm_bindgen]
pub fn des_lexpr(s: String) -> Dict {
    
}

#[wasm_bindgen]
pub fn convert_to_all(d: Dict) -> HashMap<String, String> {

}