pub mod skins;
pub mod types;

use crate::SchemaVersion;
use reqwest::Client;
use std::sync::Arc;
use crate::util::to_builder;

#[derive(Clone)]
pub struct AccountMountsBuilder {
    pub client: Client,
    pub key: Arc<Option<String>>,
    pub version: Arc<SchemaVersion>,
}

impl AccountMountsBuilder {
    to_builder!(skins, AccountSkinsBuilder);
    to_builder!(types, AccountTypesBuilder);
}
