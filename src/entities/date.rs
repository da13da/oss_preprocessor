use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{self, Deserialize, Deserializer};

pub fn deserialize_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let naive_date =
        NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S").map_err(serde::de::Error::custom)?;
    Ok(DateTime::from_utc(naive_date, Utc))
}
