pub mod cats;
pub mod nodes;

use crate::SchemaVersion;
use reqwest::Client;
use std::sync::Arc;
use crate::util::to_builder;

#[derive(Clone)]
pub struct AccountHomeBuilder {
    pub client: Client,
    pub key: Arc<Option<String>>,
    pub version: Arc<SchemaVersion>,
}

impl AccountHomeBuilder {
    to_builder!(cats, AccountHomeCatsBuilder);
    to_builder!(nodes, AccountHomeNodesBuilder);
}
