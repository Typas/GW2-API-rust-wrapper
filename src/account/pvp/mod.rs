pub mod heroes;

use crate::util::*;
use crate::SchemaVersion;
use reqwest::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct AccountPvpBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl AccountPvpBuilder {
    new_builder_from_params!();
    into_builder!(heroes, AccountPvpHeroesBuilder);
}
