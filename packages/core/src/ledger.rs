/// Keeps information about a ledger which is a wrapper for transactions.
///
/// # Example
/// ```
/// use delfin::ledger::Ledger;
///
/// let ledger = Ledger::new("TKO's trading account");
/// ```
///
///
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Ledger(String);

impl Ledger {
    pub fn new(name: &str) -> Self {
        Self(name.to_owned())
    }
}

#[cfg(test)]
mod test {
    use fake::{faker, Fake};

    use super::*;

    impl quickcheck::Arbitrary for Ledger {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            Ledger::new(&faker::company::en::CompanyName().fake::<String>())
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            quickcheck::empty_shrinker()
        }
    }
}
