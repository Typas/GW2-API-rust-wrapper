pub mod skins;
pub mod types;

use crate::util::*;
use crate::SchemaVersion;
use reqwest::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct AccountMountsBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl AccountMountsBuilder {
    new_builder_from_params!();
    into_builder!(skins, AccountSkinsBuilder);
    into_builder!(types, AccountTypesBuilder);
}
