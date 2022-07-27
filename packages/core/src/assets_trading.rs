use chrono::{DateTime, Utc};

pub trait Asset {
    fn id(&self) -> String;

    fn symbol(&self) -> String;
}

trait AssetDisposal {
    fn disposed_asset(&self) -> Box<dyn Asset>;

    fn fee_asset(&self) -> Option<Box<dyn Asset>>;

    fn executed_at(&self) -> DateTime<Utc>;
}

trait AssetAcquisition {
    fn acquired_asset(&self) -> Box<dyn Asset>;

    fn fee_asset(&self) -> Option<Box<dyn Asset>>;

    fn executed_at(&self) -> DateTime<Utc>;
}

/// Exchange expects a single asset acquired, a single asset disposed,
/// and up to one asset to capture a fee.
trait AssetExchange: AssetDisposal + AssetAcquisition {}
