pub mod heroes;

use crate::util::*;
use crate::SchemaVersion;
use reqwest::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct Builder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
}

impl Builder {
    into_builder!(heroes, heroes::Builder);
}


impl From<super::Builder> for Builder {
    fn from(source: super::Builder) -> Self {
        Self {
            client: source.client,
            key: source.key,
            version: source.version,
            url: source.url + "/pvp",
        }
    }
}
