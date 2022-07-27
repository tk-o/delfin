pub mod asset;
pub mod assets_trading;
pub mod data_sources;
pub mod ledger;
pub mod operation;
pub mod transaction;

/// Importer module for Finance on Rails suite.

#[cfg(test)]
mod tests {
    use claim::assert_ok;

    use crate::{operation, transaction::TransactionBuilder};

    #[quickcheck_macros::quickcheck]
    fn transaction_is_created_from_a_single_operation(operation: operation::Operation) {
        let tx = TransactionBuilder::default()
            .add_operation(operation)
            .build();

        assert_ok!(tx);
    }

    #[quickcheck_macros::quickcheck]
    fn transaction_is_created_from_multiple_operations(
        op1: operation::Operation,
        op2: operation::Operation,
    ) {
        let tx = TransactionBuilder::default()
            .add_operation(op1)
            .add_operation(op2)
            .build();

        assert_ok!(tx);
    }
}
