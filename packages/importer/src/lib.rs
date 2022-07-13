use std::collections::HashSet;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

mod assets_trading;

#[derive(Clone, Debug)]
struct Asset {
    id: AssetId,
}

/// International Securities Identification Number
/// https://www.investopedia.com/terms/i/isin.asp
#[derive(Clone, Debug)]
struct ISIN(String);

type TokenAddress = String;

/// Token ID
#[derive(Clone, Debug)]
struct TokenId(TokenAddress);

#[derive(Clone, Debug)]
enum FiatCurrency {
    USD,
    EUR,
}

#[derive(Clone, Debug)]
enum AssetId {
    Security(ISIN),
    Token(TokenId),
    Currency(FiatCurrency),
}

#[derive(Clone, Debug)]
struct OperationId(String);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Ledger(String);

#[derive(Clone, Debug)]
struct Operation {
    id: OperationId,
    kind: OperationKind,
    ledger: Ledger,
    asset: Asset,
    value: Decimal,
    executed_at: DateTime<Utc>,
}

#[derive(Clone, Debug)]
enum OperationKind {
    Inflow(InflowOperation),
    Outflow(OutflowOperation),
}

#[derive(Clone, Debug)]
enum InflowOperation {
    Deposit,
    Income,
    Dividend,
    Reward,
}

#[derive(Clone, Debug)]
enum OutflowOperation {
    Withdrawal,
    Cost,
    Interest,
    Donation,
}

#[derive(Debug)]
struct Transaction {
    operations: Vec<Operation>,
    ledgers: HashSet<Ledger>,
    started_at: DateTime<Utc>,
    finished_at: DateTime<Utc>,
}

#[derive(Default, Debug)]
struct TransactionBuilder {
    operations: Vec<Operation>,
    ledgers: HashSet<Ledger>,
    started_at: Option<DateTime<Utc>>,
    finished_at: Option<DateTime<Utc>>,
}

impl TransactionBuilder {
    pub fn add_operation(&mut self, operation: Operation) -> &mut Self {
        let executed_at = operation.executed_at.clone();

        self.ledgers.insert(operation.ledger.clone());

        if let Some(started_at) = self.started_at {
            if executed_at < started_at {
                self.started_at = Some(executed_at)
            }
        }

        if let Some(finished_at) = self.finished_at {
            if executed_at > finished_at {
                self.finished_at = Some(executed_at)
            }
        }

        if self.started_at.is_none() && self.finished_at.is_none() {
            self.started_at = Some(executed_at.clone());
            self.finished_at = Some(executed_at);
        }

        self.operations.push(operation);

        self
    }

    pub fn build(self) -> Result<Transaction, ()> {
        let Self {
            operations,
            ledgers,
            started_at,
            finished_at,
        } = self;

        if let (Some(started_at), Some(finished_at)) = (started_at, finished_at) {
            Ok(Transaction {
                operations,
                ledgers,
                started_at,
                finished_at,
            })
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;
    use rust_decimal_macros::dec;

    use crate::{
        Asset, AssetId, FiatCurrency, InflowOperation, Ledger, Operation, OperationId,
        OperationKind, OutflowOperation, TransactionBuilder,
    };

    #[test]
    fn it_works() {
        let main_ledger = Ledger("OkLedger".into());
        let usd_asset = Asset {
            id: AssetId::Currency(FiatCurrency::USD),
        };

        let mut txb = TransactionBuilder::default();

        let op = Operation {
            id: OperationId("OP1".into()),
            kind: OperationKind::Inflow(InflowOperation::Deposit),
            ledger: main_ledger.clone(),
            asset: usd_asset.clone(),
            value: dec!(10_000),
            executed_at: DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00")
                .unwrap()
                .into(),
        };
        txb.add_operation(op);

        let op = Operation {
            id: OperationId("OP2".into()),
            kind: OperationKind::Outflow(OutflowOperation::Cost),
            ledger: main_ledger.clone(),
            asset: usd_asset.clone(),
            value: dec!(49.99),
            executed_at: DateTime::parse_from_rfc3339("1996-12-19T16:40:01-08:00")
                .unwrap()
                .into(),
        };
        txb.add_operation(op);

        let tx = txb.build();

        assert!(tx.is_ok());

        println!("{:?}", tx);
    }
}