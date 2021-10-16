pub mod points;

use crate::util::*;
use crate::SchemaVersion;
use reqwest::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct AccountMasteryBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl AccountMasteryBuilder {
    new_builder_from_params!();
    into_builder!(points, AccountMasteryPointsBuilder);
}
