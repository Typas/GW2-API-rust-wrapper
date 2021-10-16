use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct CommercePricesData {}

impl CommercePricesData {}

#[derive(Clone)]
pub struct CommercePricesBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl CommercePricesBuilder {
    new_builder_from_params!();

    pub async fn build(self) -> ApiResult<CommercePricesData> {
        todo!()
    }

    into_builder!(id, CommercePricesIdBuilder, id: u32);
    into_builder!(ids, CommercePricesMultiIdBuilder, ids: Vec<u32>);
}

#[derive(Deserialize, Serialize)]
pub struct CommercePricesIdData {}

pub struct CommercePricesIdBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    id: u32,
}

impl CommercePricesIdBuilder {
    new_builder_from_params!(id: u32);

    pub async fn build(self) -> ApiResult<CommercePricesIdData> {
        todo!()
    }
}

pub struct CommercePricesMultiIdBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    ids: Vec<u32>,
}

impl CommercePricesMultiIdBuilder {
    new_builder_from_params!(ids: Vec<u32>);

    pub async fn build(self) -> ApiResult<Vec<CommercePricesIdData>> {
        todo!()
    }
}
