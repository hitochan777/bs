use bson::Bson;
use serde_json::{json, Value as JsonValue};

#[derive(Debug)]
pub enum ProcessError {
    FromBsonConvertError,
    FromJsonConvertError,
}



pub fn json_into_bson(json: &JsonValue) -> Result<Vec<u8>, ProcessError> {
    bson::to_bson(&json)
        .and_then(|bson| bson::to_vec(&bson))
        .and_then(|arr| Ok(arr))
        .or_else(|_| Err(ProcessError::FromJsonConvertError))
}

pub fn bson_to_simple_json(bson: &Bson) -> Result<String, ProcessError> {
    _bson_into_simple_json(bson).and_then(|res| Ok(res.to_string()))
}

fn _bson_into_simple_json(bson: &Bson) -> Result<JsonValue, ProcessError> {
    match bson {
        Bson::Double(v) if v.is_nan() => {
            let s = if v.is_sign_negative() { "-NaN" } else { "NaN" };
            Ok(json!(s))
        }
        Bson::Double(v) if v.is_infinite() => {
            let s = if v.is_sign_negative() {
                "-Infinity"
            } else {
                "Infinity"
            };
            Ok(json!(s))
        }
        Bson::String(v) => Ok(json!(v)),
        Bson::Array(v) => v
            .into_iter()
            .map(_bson_into_simple_json)
            .collect::<Result<Vec<_>, ProcessError>>()
            .and_then(|arr| Ok(JsonValue::Array(arr))),
        Bson::Document(v) => v
            .into_iter()
            .map(|(k, v)| _bson_into_simple_json(v).and_then(|json| Ok((k.clone(), json))))
            .collect::<Result<Vec<_>, ProcessError>>()
            .and_then(|json| Ok(JsonValue::Object(serde_json::Map::from_iter(json)))),
        Bson::Boolean(v) => Ok(json!(v)),
        Bson::Null => Ok(JsonValue::Null),
        Bson::Double(v) => {
            let s = if v.is_sign_negative() { "-NaN" } else { "NaN" };
            Ok(json!(s))
        }
        Bson::Int32(v) => Ok(json!(v)),
        Bson::Int64(v) => Ok(json!(v)),
        _ => Err(ProcessError::FromBsonConvertError),
    }
}
