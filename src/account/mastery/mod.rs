pub mod points;

use crate::SchemaVersion;
use reqwest::Client;
use std::sync::Arc;
use crate::util::to_builder;

#[derive(Clone)]
pub struct AccountMasteryBuilder {
    pub client: Client,
    pub key: Arc<Option<String>>,
    pub version: Arc<SchemaVersion>,
}

impl AccountMasteryBuilder {
    to_builder!(points, AccountMasteryPointsBuilder);
}
