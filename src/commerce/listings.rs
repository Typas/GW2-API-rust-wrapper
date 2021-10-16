use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct CommerceListingsData {}

impl CommerceListingsData {}

#[derive(Clone)]
pub struct CommerceListingsBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl CommerceListingsBuilder {
    new_builder_from_params!();

    pub async fn build(self) -> ApiResult<CommerceListingsData> {
        todo!()
    }

    into_builder!(id, CommerceListingsIdBuilder, id: u32);
    into_builder!(ids, CommerceListingsMultiIdBuilder, ids: Vec<u32>);
}

#[derive(Deserialize, Serialize)]
pub struct CommerceListingsIdData {}

pub struct CommerceListingsIdBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    id: u32,
}

impl CommerceListingsIdBuilder {
    new_builder_from_params!(id: u32);

    pub async fn build(self) -> ApiResult<CommerceListingsIdData> {
        todo!()
    }
}

pub struct CommerceListingsMultiIdBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    ids: Vec<u32>,
}

impl CommerceListingsMultiIdBuilder {
    new_builder_from_params!(ids: Vec<u32>);

    pub async fn build(self) -> ApiResult<Vec<CommerceListingsIdData>> {
        todo!()
    }
}
