use serde::{Deserialize, Deserializer};
use serde_json;

pub fn string_to_f32<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error> 
where
    D: Deserializer<'de>,
{
    let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

    match value {
        serde_json::Value::Null => Ok(None),
        serde_json::Value::String(s) => {
            let s = s.replace(",", "");
            let x = s.parse::<f32>().map_err(|e| format!("Failed to parse '{}': {}", s, e));
            let x = match x {
                Ok(z) => Some(z),
                Err(_) => None,
            };
            Ok(x)
        },
        _ => Err(serde::de::Error::custom("String value could not be converted to f32"))
    }
}

pub fn deserialize_difference_percentage<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize the field as a serde_json::Value to handle different types
    let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

    // Match on the type of value and convert accordingly
    match value {
        serde_json::Value::Number(num) => {
            // Convert to f32 if it's a number
            Ok(Some(num.as_f64().map(|v| v as f32).unwrap()))
        },
        serde_json::Value::String(s) => {
            // Handle special strings like "Infinity"
            if s == "Infinity" {
                Ok(Some(f32::INFINITY))
            } else if s == "-Infinity" {
                Ok(Some(f32::NEG_INFINITY))
            } else {
                // Try to parse as a float
                s.parse::<f32>().map(Some).map_err(|_| serde::de::Error::custom("Invalid string format"))
            }
        },
        serde_json::Value::Null => Ok(None), // Handle null values
        _ => Err(serde::de::Error::custom("Invalid type for differencePercentage")),
    }
}