use std::collections::HashSet;

use chrono::{DateTime, Utc};

use crate::{operation::Operation, ledger::Ledger};


#[derive(Clone, Debug)]
pub struct Transaction {
    pub operations: Vec<Operation>,
    pub ledgers: HashSet<Ledger>,
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
}

#[derive(Default, Debug)]
pub struct TransactionBuilder {
    operations: Vec<Operation>,
    ledgers: HashSet<Ledger>,
    started_at: Option<DateTime<Utc>>,
    finished_at: Option<DateTime<Utc>>,
}

impl TransactionBuilder {
    pub fn add_operation(mut self, operation: Operation) -> Self {
        let executed_at = operation.executed_at.to_owned();

        self.ledgers.insert(operation.ledger.to_owned());

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
            self.started_at = Some(executed_at.to_owned());
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
    use quickcheck::Arbitrary;

    use super::*;

    impl quickcheck::Arbitrary for Transaction {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            TransactionBuilder::default()
                .add_operation(Arbitrary::arbitrary(g))
                .add_operation(Arbitrary::arbitrary(g))
                .build()
                .unwrap()
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            quickcheck::empty_shrinker()
        }
    }
}
