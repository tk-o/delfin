use core::fmt;

#[derive(Clone, Debug)]
pub struct Asset {
    pub id: AssetId,
    pub name: AssetName,
}

#[derive(Clone, Debug)]
pub enum AssetId {
    Security(ISIN),
    Token(TokenId),
    Currency(FiatCurrency),
}

pub type AssetName = String;

/// International Securities Identification Number
/// https://www.investopedia.com/terms/i/isin.asp
#[derive(Clone, Debug)]
pub struct ISIN(pub String);

/// Token ID
#[derive(Clone, Debug)]
pub struct TokenId(pub String);

#[derive(Clone, Debug)]
pub enum FiatCurrency {
    USD,
    EUR,
}

impl fmt::Display for FiatCurrency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use fake::{
        faker::{
            company::en::{BsAdj, BsNoun, CompanyName},
            number::en::NumberWithFormat,
        },
        Fake,
    };
    use quickcheck::Arbitrary;

    use super::*;

    impl quickcheck::Arbitrary for AssetId {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            g.choose(&[
                AssetId::Currency(FiatCurrency::EUR),
                AssetId::Currency(FiatCurrency::USD),
                AssetId::Token(TokenId(NumberWithFormat(&"0x####...####").fake())),
                AssetId::Security(ISIN(NumberWithFormat(&"###-###-###").fake())),
            ])
            .unwrap()
            .to_owned()
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
                }
                AssetId::Currency(c) => c.to_string(),
            };

            Self { id, name }
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            quickcheck::empty_shrinker()
        }
    }
}
