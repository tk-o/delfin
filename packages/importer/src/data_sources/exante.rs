use std::{error::Error, fmt::Debug, fs, path::Path};

use chrono::{DateTime, TimeZone, Utc};
use csv::ReaderBuilder;
use serde::{Deserialize, Deserializer};

pub fn read_csv_file<TPath>(file_path: TPath) -> Result<Vec<Record>, Box<dyn Error>>
where
    TPath: AsRef<Path> + Debug,
{
    let data = fs::read_to_string(file_path)?;

    let mut rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(data.as_bytes());

    let records = rdr
        .deserialize::<Record>()
        .filter_map(|record| record.ok())
        .collect();

    Ok(records)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Record {
    #[serde(rename = "Transaction ID")]
    tx_id: String,

    #[serde(rename = "Account ID")]
    account_id: String,

    #[serde(rename = "Symbol ID")]
    symbol_id: String,

    #[serde(rename = "ISIN")]
    isin: String,

    #[serde(rename = "Operation type")]
    operation_type: String,

    #[serde(rename = "When", deserialize_with = "deserialize_exante_date")]
    when: chrono::DateTime<chrono::Utc>,

    #[serde(rename = "Sum")]
    sum: f32,

    #[serde(rename = "Asset")]
    asset: String,

    #[serde(rename = "UUID")]
    uuid: String,
}

const EXANTE_DATE_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

// The signature of a deserialize_with function must follow the pattern:
//
//    fn deserialize<'de, D>(D) -> Result<T, D::Error>
//    where
//        D: Deserializer<'de>
//
// although it may also be generic over the output types T.
pub fn deserialize_exante_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Utc.datetime_from_str(&s, EXANTE_DATE_FORMAT)
        .map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use claim::{assert_gt, assert_ok};

    use super::*;

    #[test]
    fn load_file_contents() {
        let operations = read_csv_file(Path::new("input/exante/demo.csv"));

        assert_ok!(&operations);

        let operations = operations.unwrap();

        assert_gt!(operations.len(), 0);
    }
}
