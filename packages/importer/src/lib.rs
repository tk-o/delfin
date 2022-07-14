use std::{collections::HashSet, fmt};

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

mod assets_trading;

type AssetName = String;

#[derive(Clone, Debug)]
struct Asset {
    id: AssetId,
    name: AssetName,
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

impl fmt::Display for FiatCurrency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
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
    pub fn add_operation(mut self, operation: Operation) -> Self {
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

    pub fn build(self) -> Result<Transaction, String> {
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
            Err("Missing dates".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use claim::assert_ok;
    use fake::{faker::{self, company::{ en::{Buzzword, CompanyName, CompanySuffix, BsAdj, BsNoun}}, number::en::NumberWithFormat, chrono::raw::DateTimeBefore}, locales::EN, Fake, StringFaker};
    use quickcheck::Arbitrary;
    use rust_decimal_macros::dec;

    use crate::{
        Asset, AssetId, FiatCurrency, InflowOperation, Ledger, Operation, OperationId,
        OperationKind, TransactionBuilder, TokenId, ISIN,
    };

    #[derive(Clone, Debug)]
    struct DateTimeUtc(DateTime<Utc>);

    impl quickcheck::Arbitrary for DateTimeUtc {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            let datetime: DateTime<Utc> = DateTimeBefore(EN, Utc::now()).fake();

            Self(datetime)
        }
        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            quickcheck::empty_shrinker()
        }
    }

    impl quickcheck::Arbitrary for Ledger {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            Self(faker::company::en::CompanyName().fake())
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            quickcheck::empty_shrinker()
        }
    }

    impl quickcheck::Arbitrary for AssetId {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            g.choose(&[
                AssetId::Currency(FiatCurrency::EUR),
                AssetId::Currency(FiatCurrency::USD),
                AssetId::Token(TokenId(
                    NumberWithFormat(&"0x####...####").fake()
                )),
                AssetId::Security(ISIN(
                    NumberWithFormat(&"###-###-###").fake()
                ))
            ]).unwrap().to_owned()
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            quickcheck::empty_shrinker()
        }
    }

    impl quickcheck::Arbitrary for Asset {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let id: AssetId = Arbitrary::arbitrary(g);
            let name: String = match &id {
                AssetId::Security(_) => CompanyName().fake(),
                AssetId::Token(_) => {
                    let n1: String = BsAdj().fake();
                    let n2: String = BsNoun().fake();

                    format!("{} {} Chain", n1, n2)
                },
                AssetId::Currency(c) => c.to_string(),
            };

            Self {
                id,
                name,
            }
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            quickcheck::empty_shrinker()
        }
    }

    fn create_random_operation() -> Operation {
        let main_ledger = Ledger("OkLedger".into());
        let usd_asset = Asset {
            id: AssetId::Currency(FiatCurrency::USD),
            name: "Asset Name".into(),
        };

        Operation {
            id: OperationId("OP1".into()),
            kind: OperationKind::Inflow(InflowOperation::Deposit),
            ledger: main_ledger.clone(),
            asset: usd_asset.clone(),
            value: dec!(10_000),
            executed_at: DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00")
                .unwrap()
                .into(),
        }
    }

    #[test]
    fn transaction_is_created_from_a_single_operation() {
        let tx = TransactionBuilder::default()
            .add_operation(create_random_operation())
            .build();

        assert_ok!(tx);
    }

    #[quickcheck_macros::quickcheck]
    fn transaction_is_created_from_multiple_operations(dates: Vec<Asset>) {
        println!("{:?}", dates);
        let tx = TransactionBuilder::default()
            .add_operation(create_random_operation())
            .add_operation(create_random_operation())
            .add_operation(create_random_operation())
            .add_operation(create_random_operation())
            .build();

        assert_ok!(tx);
    }
}
