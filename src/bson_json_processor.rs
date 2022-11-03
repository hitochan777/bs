use bson::{Bson};
use serde_json::{json, Value as JsonValue};

#[derive(Debug, PartialEq)]
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
        Bson::Double(v) => {
            Ok(json!(v))
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
        Bson::Int32(v) => Ok(json!(v)),
        Bson::Int64(v) => Ok(json!(v)),
        _ => Err(ProcessError::FromBsonConvertError),
    }
}

#[cfg(test)]
mod tests {
    use bson::{Bson, bson, DateTime};
    use super::{bson_to_simple_json, ProcessError};
    #[test]
    fn it_encodes_positive_double_nan() {
      let bson = Bson::Double(f64::NAN);
      let json_str = bson_to_simple_json(&bson);
      assert_eq!(json_str, Ok(String::from("\"NaN\"")));
    }

    #[test]
    fn it_encodes_negative_double_nan() {
      let bson = bson!(-f64::NAN);
      let json_str = bson_to_simple_json(&bson);
      assert_eq!(json_str, Ok(String::from("\"-NaN\"")));
    }

    #[test]
    fn it_encodes_positive_double_infinity() {
      let bson = bson!(f64::INFINITY);
      let json_str = bson_to_simple_json(&bson);
      assert_eq!(json_str, Ok(String::from("\"Infinity\"")));
    }

    #[test]
    fn it_encodes_negative_double_infinity() {
      let bson = bson!(-f64::INFINITY);
      let json_str = bson_to_simple_json(&bson);
      assert_eq!(json_str, Ok(String::from("\"-Infinity\"")));
    }

    #[test]
    fn it_encodes_double() {
      let bson = bson!(3.4);
      let json_str = bson_to_simple_json(&bson);
      assert_eq!(json_str, Ok(String::from("3.4")));
    }

    #[test]
    fn it_encodes_string() {
      let bson = bson!("foo");
      let json_str = bson_to_simple_json(&bson);
      assert_eq!(json_str, Ok(String::from("\"foo\"")));
    }

    #[test]
    fn it_encodes_array() {
      let bson = bson!([1,2,"bar"]);
      let json_str = bson_to_simple_json(&bson);
      assert_eq!(json_str, Ok(String::from("[1,2,\"bar\"]")));
    }

    #[test]
    fn it_encodes_document() {
      let bson = bson!(
        {
          "a": 2,
          "b": 3,
        }
      );
      let json_str = bson_to_simple_json(&bson);
      assert_eq!(json_str, Ok(String::from("{\"a\":2,\"b\":3}")));
    }

    #[test]
    fn it_encodes_boolean() {
      let bson = bson!(true);
      let json_str = bson_to_simple_json(&bson);
      assert_eq!(json_str, Ok(String::from("true")));
    }
    
    #[test]
    fn it_encodes_null() {
      let bson = bson!(null);
      let json_str = bson_to_simple_json(&bson);
      assert_eq!(json_str, Ok(String::from("null")));
    }

    #[test]
    fn it_encodes_int32() {
      let bson = Bson::Int32(3);
      let json_str = bson_to_simple_json(&bson);
      assert_eq!(json_str, Ok(String::from("3")));
    }

    #[test]
    fn it_encodes_int64() {
      let bson = Bson::Int64(3313232321321321312);
      let json_str = bson_to_simple_json(&bson);
      assert_eq!(json_str, Ok(String::from("3313232321321321312")));
    }

    #[test]
    fn it_returns_error_otherwise() {
      let bson = Bson::DateTime(DateTime::from_millis(1667457796000));
      let json_str = bson_to_simple_json(&bson);
      assert_eq!(json_str, Err(ProcessError::FromBsonConvertError));
    }
}