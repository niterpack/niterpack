use serde::de::Unexpected;
use serde_json::Value;

pub trait FromValueExt<'a> {
    fn from_value(value: &'a Value) -> Self;
}

impl<'a> FromValueExt<'a> for Unexpected<'a> {
    fn from_value(value: &'a Value) -> Self {
        match value {
            Value::String(o) => Unexpected::Str(o),
            Value::Array(_) => Unexpected::Seq,
            Value::Bool(o) => Unexpected::Bool(*o),
            Value::Null => Unexpected::Other("null"),
            Value::Object(_) => Unexpected::Map,
            Value::Number(o) => {
                if o.is_i64() {
                    Unexpected::Signed(o.as_i64().unwrap())
                } else if o.is_u64() {
                    Unexpected::Unsigned(o.as_u64().unwrap())
                } else {
                    Unexpected::Float(o.as_f64().unwrap())
                }
            }
        }
    }
}
