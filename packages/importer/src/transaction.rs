use std::{collections::HashSet, ops::Deref};

use chrono::{DateTime, Utc};

use crate::{ledger::Ledger, operation::Operation};

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
    pub fn add_operation(&mut self, operation: Operation) -> &mut Self {
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

    pub fn build(&mut self) -> Result<Transaction, String> {
        let Self {
            operations,
            ledgers,
            started_at,
            finished_at,
        } = self;

        if operations.is_empty() {
            return Err("Missing operations".into());
        }

        if let (Some(started_at), Some(finished_at)) = (started_at, finished_at) {
            Ok(Transaction {
                operations: self.operations.to_owned(),
                ledgers: self.ledgers.to_owned(),
                started_at: started_at.to_owned(),
                finished_at: finished_at.to_owned(),
            })
        } else {
            Err("Missing dates".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use claim::{assert_err, assert_ok};

    use super::*;

    #[test]
    fn builder_returns_error_when_no_operations_provided() {
        let tx = TransactionBuilder::default().build();

        assert_err!(tx);
    }

    #[quickcheck_macros::quickcheck]
    fn builder_returns_tx_when_one_operation_provided(operation: Operation) {
        let tx = TransactionBuilder::default()
            .add_operation(operation)
            .build();

        assert_ok!(tx);
    }

    #[quickcheck_macros::quickcheck]
    fn builder_returns_tx_when_multiple_operations_provided(operations: Vec<Operation>) {
        // sometimes there's no sample provided
        if operations.is_empty() {
            return ;
        }

        let mut tx_builder = TransactionBuilder::default();

        for operation in operations.into_iter().take(4) {
            tx_builder.add_operation(operation);
        }

        let tx = tx_builder.build();

        assert_ok!(tx);
    }
}
