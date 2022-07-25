#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Ledger(pub String);

#[cfg(test)]
mod test {
    use fake::{faker, Fake};

    use super::*;

    impl quickcheck::Arbitrary for Ledger {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            Self(faker::company::en::CompanyName().fake())
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            quickcheck::empty_shrinker()
        }
    }
}
