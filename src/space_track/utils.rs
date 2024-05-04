use serde::{de, Deserialize, Deserializer};
use serde_json::Value;

pub fn deserialize_optional_string_to_i64<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<i64>, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => Some(s.parse().map_err(de::Error::custom)?),
        Value::Number(num) => Some(
            num.as_i64()
                .ok_or_else(|| de::Error::custom("Invalid number"))?,
        ),
        Value::Null => None,
        _ => return Err(de::Error::custom("wrong type")),
    })
}

pub fn deserialize_string_to_i64<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<i64, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().map_err(de::Error::custom)?,
        Value::Number(num) => num
            .as_i64()
            .ok_or_else(|| de::Error::custom("Invalid number"))?,
        _ => return Err(de::Error::custom("wrong type")),
    })
}
