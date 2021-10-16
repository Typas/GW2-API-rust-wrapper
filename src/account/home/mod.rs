pub mod cats;
pub mod nodes;

use crate::util::*;
use crate::SchemaVersion;
use reqwest::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct AccountHomeBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl AccountHomeBuilder {
    new_builder_from_params!();
    into_builder!(cats, AccountHomeCatsBuilder);
    into_builder!(nodes, AccountHomeNodesBuilder);
}
