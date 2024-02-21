use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use js_sys::Error;
use toml::Value;

type JsResult = Result<JsValue, Error>;

// no need to deserialize json bc of js but still gonna do it anyway
#[wasm_bindgen]
pub fn des_json(s: String) -> JsResult {
    let v: Value = serde_json::from_str(&s)
        .map_err(|_| Error::new("Invalid JSON"))?;

    Ok(serde_wasm_bindgen::to_value(&v).unwrap())
}

#[wasm_bindgen]
pub fn des_toml(s: String) -> JsResult {
    let v: Value = (&s)
        .parse()
        .map_err(|_| Error::new("Invalid TOML"))?;

    Ok(serde_wasm_bindgen::to_value(&v).unwrap())
}

#[wasm_bindgen]
pub fn des_ron(s: String) -> JsResult {
    let v: Value = ron::from_str(&s)
        .map_err(|_| Error::new("Invalid RON"))?;

    Ok(serde_wasm_bindgen::to_value(&v).unwrap())
}

#[wasm_bindgen]
pub fn des_yaml(s: String) -> JsResult {
    let v: Value = serde_yaml::from_str(&s)
        .map_err(|_| Error::new("Invalid YAML"))?;

    Ok(serde_wasm_bindgen::to_value(&v).unwrap())
}

#[wasm_bindgen]
pub fn des_qs(s: String) -> JsResult {
    let v: Value = serde_qs::from_str(&s)
        .map_err(|_| Error::new("Invalid QString"))?;

    Ok(serde_wasm_bindgen::to_value(&v).unwrap())
}

#[wasm_bindgen]
pub fn des_lexpr(s: String) -> JsResult {
    let v: Value = serde_lexpr::from_str(&s)
        .map_err(|_| Error::new("Invalid LExpr"))?;

    Ok(serde_wasm_bindgen::to_value(&v).unwrap())
}

#[wasm_bindgen]
pub fn convert_to_all(d: JsValue) -> JsResult {
    let v: Value = serde_wasm_bindgen::from_value(d).unwrap();

    let mut m = HashMap::new();

    m.insert("json", serde_json::to_string(&v).unwrap());
    m.insert("toml", toml::to_string(&v).unwrap());
    m.insert("ron", ron::to_string(&v).unwrap());
    m.insert("yaml", serde_yaml::to_string(&v).unwrap());
    m.insert("qstr", serde_qs::to_string(&v).unwrap());
    m.insert("lexpr", serde_lexpr::to_string(&v).unwrap());

    Ok(serde_wasm_bindgen::to_value(&m).unwrap())
}