pub mod coins;
pub mod gems;

use crate::util::*;
use crate::SchemaVersion;
use reqwest::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct CommerceExchangeBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl CommerceExchangeBuilder {
    new_builder_from_params!();
    into_builder!(coins, CommerceExchangeCoinsBuilder);
    into_builder!(gems, CommerceExchangeGemsBuilder);
}
