pub mod delivery;
pub mod exchange;
pub mod listings;
pub mod prices;
pub mod transcations;

use crate::api::NotAuthenticatedError;
use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct CommerceBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl CommerceBuilder {
    new_builder_from_params!();

    into_builder!(delivery, CommerceDeliveryBuilder);
    into_builder!(exchange, CommerceExchangeBuilder);
    into_builder!(listings, CommerceListingsBuilder);
    into_builder!(prices, CommercePricesBuilder);
    into_builder!(transcations, CommerceTranscationsBuilder);
}
