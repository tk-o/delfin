use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use crate::{ledger::Ledger, asset::Asset};

#[derive(Clone, Debug)]
pub struct Operation {
    pub id: OperationId,
    pub kind: OperationKind,
    pub ledger: Ledger,
    pub asset: Asset,
    pub value: Decimal,
    pub executed_at: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct OperationId(pub String);

#[derive(Clone, Debug)]
pub enum OperationKind {
    Inflow(InflowOperation),
    Outflow(OutflowOperation),
}

#[derive(Clone, Debug)]
pub enum InflowOperation {
    Deposit,
    Income,
    Dividend,
    Reward,
}

#[derive(Clone, Debug)]
pub enum OutflowOperation {
    Withdrawal,
    Cost,
    Interest,
    Donation,
}

#[cfg(test)]
pub(crate) mod test {
    use std::str::FromStr;

    use chrono::Duration;
    use fake::{faker, Fake};
    use quickcheck::Arbitrary;

    use super::*;

    impl quickcheck::Arbitrary for OperationId {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            Self(faker::number::en::NumberWithFormat("OP####").fake())
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            quickcheck::empty_shrinker()
        }
    }

    impl quickcheck::Arbitrary for InflowOperation {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            g.choose(&[Self::Deposit, Self::Dividend, Self::Income, Self::Reward])
                .unwrap()
                .to_owned()
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            quickcheck::empty_shrinker()
        }
    }

    impl quickcheck::Arbitrary for OutflowOperation {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            g.choose(&[Self::Cost, Self::Donation, Self::Interest, Self::Withdrawal])
                .unwrap()
                .to_owned()
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            quickcheck::empty_shrinker()
        }
    }

    impl quickcheck::Arbitrary for OperationKind {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let seed: u8 = g.choose(&[0, 1]).unwrap().to_owned();

            if seed == 0 {
                Self::Inflow(Arbitrary::arbitrary(g))
            } else {
                Self::Outflow(Arbitrary::arbitrary(g))
            }
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            quickcheck::empty_shrinker()
        }
    }

    impl quickcheck::Arbitrary for Operation {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let days_count = g.choose(&(0..1_000).collect::<Vec<_>>()).unwrap().to_owned();

            let int_part: u16 = g.choose(&(0..1_000).collect::<Vec<_>>()).unwrap().to_owned();
            let decimal_part: u16 = g.choose(&(0..100).collect::<Vec<_>>()).unwrap().to_owned();

            let value_str = format!("{}.{}", &int_part, &decimal_part);

            let value: Decimal = Decimal::from_str(&value_str).unwrap_or_default();

            Self {
                id: Arbitrary::arbitrary(g),
                kind: Arbitrary::arbitrary(g),
                ledger: Arbitrary::arbitrary(g),
                asset: Arbitrary::arbitrary(g),
                executed_at: faker::chrono::en::DateTimeBetween(
                    Utc::now().checked_sub_signed(Duration::days(days_count)).unwrap(),
                    Utc::now(),
                ).fake(),
                value,
            }
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            quickcheck::empty_shrinker()
        }
    }
}
