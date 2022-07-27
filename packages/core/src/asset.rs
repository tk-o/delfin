use core::fmt;
use std::str::FromStr;

use regex::Regex;
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct Asset {
    id: AssetId,
    name: AssetName,
}

impl Asset {
    pub fn new(id: AssetId, name: AssetName) -> Self {
        Self { id, name }
    }
}

#[derive(Clone, Debug)]
pub enum AssetId {
    Security(ISIN),
    Token(TokenId),
    Currency(FiatCurrency),
}

pub type AssetName = String;

/// International Securities Identification Number
/// <https://www.investopedia.com/terms/i/isin.asp>
///
/// # Example
/// ```
/// use std::str::FromStr;
/// use finance_on_rails_importer::asset::{ISIN, ISINError};
///
/// let isin = "NA-000K0VF05-4".parse::<ISIN>();
/// assert!(isin.is_ok());
///
/// let isin = "A-000K0VF05".parse::<ISIN>();
/// assert!(matches!(isin.unwrap_err(), ISINError::InvalidISO6166));
/// ```
#[derive(Clone, Debug)]
pub struct ISIN(String);

#[derive(Debug, Error)]
pub enum ISINError {
    #[error("Invalid regex")]
    Regex,

    #[error("Invalid ISO 6166")]
    InvalidISO6166,
}

impl FromStr for ISIN {
    type Err = ISINError;

    /// Parses a string according to the ISO 6166:
    /// International Securities Identification Number (ISIN)
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized_value = s.replace('-', "");

        // Naive regex for ISO 6166-compatible value
        let iso6166_regex = r"^[A-Z]{2}[\dA-Z]{10}$"
            .parse::<Regex>()
            .map_err(|_| ISINError::Regex)?;

        if !iso6166_regex.is_match(&normalized_value) {
            return Err(ISINError::InvalidISO6166);
        }

        Ok(ISIN(s.into()))
    }
}

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
mod test {
    use claim::{assert_err, assert_ok};

    use super::*;

    #[test]
    fn can_parse_valid_isin_input() {
        let valid_isin_numbers = vec![
            "NA-000K0VF05-4",
            "NA000K0VF054",
            "US-000402625-0",
            "US0004026250",
        ];

        valid_isin_numbers.into_iter().for_each(|isin_number| {
            assert_ok!(isin_number.parse::<ISIN>());
        });
    }

    #[test]
    fn cannot_parse_invalid_isin_input() {
        let valid_isin_numbers = vec![
            "NA-000K0VF05!4",
            "NA000K0VF0544",
            "RAEA000K0VF054",
            "000402625-000",
            "US00040262500",
        ];

        valid_isin_numbers.into_iter().for_each(|isin_number| {
            assert_err!(isin_number.parse::<ISIN>());
        });
    }
}

#[cfg(test)]
mod prop_tests {
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
