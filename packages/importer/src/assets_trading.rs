use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AssetBasicInfo {
    id: String,
    symbol: String,
}

impl AssetBasicInfo {
    pub fn new(id: &str, symbol: &str) -> Self {
        Self {
            id: id.into(),
            symbol: symbol.into(),
        }
    }
}

pub trait Asset {
    fn id(&self) -> String;

    fn symbol(&self) -> String;
}

impl Asset for AssetBasicInfo {
    fn id(&self) -> String {
        self.id.to_owned()
    }

    fn symbol(&self) -> String {
        self.symbol.to_owned()
    }
}

trait AssetTrade {
    fn disposed_asset(&self) -> Box<dyn Asset>;

    fn acquired_asset(&self) -> Box<dyn Asset>;

    fn fee_asset(&self) -> Option<Box<dyn Asset>>;

    fn executed_at(&self) -> DateTime<Utc>;
}

#[cfg(test)]
mod tests {
    use super::{Asset, AssetBasicInfo };

    #[test]
    fn it_can_load_data_from_json() {
        let mut test_data = test_data();
        let asset = test_data.pop().unwrap();

        assert_eq!(asset.id(), "US92189F7915");
        assert_eq!(asset.symbol(), "GDXJ.ARCA");
    }

    fn test_data() -> Vec<AssetBasicInfo> {
        let data = r#"
            [
                {
                    "id": "US92189F7915",
                    "symbol": "GDXJ.ARCA"
                }
            ]
        "#;

        serde_json::from_str(data).unwrap()
    }
}
