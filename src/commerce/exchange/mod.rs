pub mod coins;
pub mod gems;

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
    into_builder!(coins, coins::Builder);
    into_builder!(gems, gems::Builder);
}

impl From<super::Builder> for Builder {
    fn from(source: super::Builder) -> Self {
        Self {
            client: source.client,
            key: source.key,
            version: source.version,
            url: source.url + "/exchange",
        }
    }
}
