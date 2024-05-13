use std::str::FromStr;

use serde::{de, Deserialize, Deserializer};
use serde_json::Value;

use crate::Direction;

fn deserialize_string<'de, D: Deserializer<'de>, T: FromStr>(deserializer: D) -> Result<T, D::Error>
where
    <T as FromStr>::Err: std::fmt::Display,
{
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().map_err(de::Error::custom)?,
        _ => return Err(de::Error::custom("wrong type")),
    })
}

fn deserialize_optional_string<'de, D: Deserializer<'de>, T: FromStr>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    <T as FromStr>::Err: std::fmt::Display,
{
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => Some(s.parse().map_err(de::Error::custom)?),
        Value::Null => None,
        _ => return Err(de::Error::custom("wrong type")),
    })
}

pub fn deserialize_optional_string_to_u64<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<u64>, D::Error> {
    deserialize_optional_string(deserializer)
}

pub fn deserialize_string_to_u64<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<u64, D::Error> {
    deserialize_string(deserializer)
}

pub fn deserialize_string_to_f64<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<f64, D::Error> {
    deserialize_string(deserializer)
}

pub fn deserialize_string_to_u8<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<u8, D::Error> {
    deserialize_string(deserializer)
}

pub fn deserialize_optional_string_to_u8<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<u8>, D::Error> {
    deserialize_optional_string(deserializer)
}

pub fn deserialize_optional_string_to_f64<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<f64>, D::Error> {
    deserialize_optional_string(deserializer)
}

pub fn deserialize_string_to_bool<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<bool, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s == "Y",
        _ => return Err(de::Error::custom("wrong type")),
    })
}

pub fn deserialize_string_to_direction<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Direction, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => match s.as_str() {
            "ascending" => Direction::Ascending,
            "descending" => Direction::Descending,
            _ => return Err(de::Error::custom("wrong value")),
        },
        _ => return Err(de::Error::custom("wrong type")),
    })
}
