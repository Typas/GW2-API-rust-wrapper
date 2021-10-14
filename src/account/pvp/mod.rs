pub mod heroes;

use crate::SchemaVersion;
use reqwest::Client;
use std::sync::Arc;
use crate::util::to_builder;

#[derive(Clone)]
pub struct AccountPvpBuilder {
    pub client: Client,
    pub key: Arc<Option<String>>,
    pub version: Arc<SchemaVersion>,
}

impl AccountPvpBuilder {
    to_builder!(heroes, AccountPvpHeroesBuilder);
}
