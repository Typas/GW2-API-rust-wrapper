pub mod delivery;
pub mod exchange;
pub mod listings;
pub mod prices;
pub mod transcations;

use crate::api::NotAuthenticatedError;
use crate::util::*;
use crate::{ApiClient, ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct Builder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
}

impl Builder {
    into_builder!(delivery, delivery::Builder);
    into_builder!(exchange, exchange::Builder);
    into_builder!(listings, listings::Builder);
    into_builder!(prices, prices::Builder);
    into_builder!(transcations, transcations::Builder);
}

impl From<&ApiClient> for Builder {
    fn from(source: &ApiClient) -> Self {
        Self {
            client: source.client.clone(),
            key: source.key.clone(),
            version: source.version.clone(),
            url: "https://api.guildwars2.com/v2/commerce".to_string(),
        }
    }
}
