use std::{error::Error, fmt::Debug, fs, path::Path};

use chrono::{DateTime, TimeZone, Utc};
use csv::ReaderBuilder;
use serde::{Deserialize, Deserializer};
use slice_group_by::GroupBy;
use thiserror::Error;

use crate::{
    asset::{Asset, AssetId, FiatCurrency, ISINError, ISIN},
    ledger::Ledger,
    operation::{
        InflowOperation, Operation, OperationId, OperationIdError, OperationKind,
        OutflowOperation,
    },
    transaction::{Transaction, TransactionBuilder},
};

pub fn read_csv_file<TPath>(file_path: TPath) -> Result<Vec<RawRecord>, Box<dyn Error>>
where
    TPath: AsRef<Path> + Debug,
{
    let data = fs::read_to_string(file_path)?;

    let mut rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(data.as_bytes());

    let records = rdr
        .deserialize::<RawRecord>()
        .filter_map(|record| record.ok())
        .collect();

    Ok(records)
}

pub fn group_records_into_transactions(
    records: &[RawRecord],
) -> Result<Vec<Transaction>, RawRecordError> {
    Ok(records
        .linear_group_by(|a, b| a.when == b.when)
        .filter_map(|group| {
            let mut tx_builder = TransactionBuilder::default();

            for record in group {
                tx_builder.add_operation(record.try_into().ok()?);
            }

            tx_builder.build().ok()
        })
        .collect::<Vec<_>>())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RawRecord {
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

#[derive(Error, Debug)]
pub enum RawRecordError {
    #[error("{0}")]
    OperationId(#[from] OperationIdError),

    #[error("{0}")]
    ISIN(#[from] ISINError),

    #[error("Invalid record value")]
    Value(#[from] rust_decimal::Error),
}

impl<'a> TryInto<Operation> for &'a RawRecord {
    type Error = RawRecordError;

    fn try_into(self) -> Result<Operation, Self::Error> {
        // TODO: assign exact operation kind
        let kind = if self.sum > 0.0 {
            OperationKind::Inflow(InflowOperation::Deposit)
        } else {
            OperationKind::Outflow(OutflowOperation::Withdrawal)
        };

        let asset_id = if &self.isin != "None" {
            AssetId::Security(self.isin.parse::<ISIN>()?)
        } else {
            // TODO: map the currency
            AssetId::Currency(FiatCurrency::USD)
        };

        Ok(Operation {
            id: self.uuid.parse::<OperationId>()?,
            kind,
            ledger: Ledger::new(self.account_id.as_str()),
            asset: Asset::new(asset_id, self.asset.to_owned()),
            value: self.sum.abs().try_into()?,
            executed_at: self.when,
        })
    }
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
    use claim::{assert_gt, assert_ok};

    use super::*;

    static DEMO_CSV_FILE_PATH: &str = "input/exante/demo.csv";

    #[test]
    fn load_file_contents() {
        let operations = read_csv_file(Path::new(DEMO_CSV_FILE_PATH));

        assert_ok!(&operations);

        let operations = operations.unwrap();

        assert_gt!(operations.len(), 0);
    }

    #[test]
    fn group_records() {
        /**
         * Think of using a state machine for the following transitions:
         *
         * Initial State
         * \
         *  - External Data (x) importer process -> Completed Import State
         *
         * Completed Import State
         * \
         *  - Internal Raw Data (x) aggregation process -> Completed Transform State
         *
         * Completed Transform State
         * \
         *  - Internal Aggregated Data (x) accounting process -> ?
         */
        let records = read_csv_file(Path::new(DEMO_CSV_FILE_PATH))
            .expect("Could not load the CSV file");

        let groupped_records = group_records_into_transactions(&records);

        println!("{:#?}", groupped_records);
    }
}
