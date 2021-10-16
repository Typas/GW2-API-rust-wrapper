use crate::api::NotAuthenticatedError;
use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct CommerceTranscationsData {}

impl CommerceTranscationsData {}

#[derive(Clone)]
pub struct CommerceTranscationsBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl CommerceTranscationsBuilder {
    new_builder_from_params!();

    pub async fn build(self) -> ApiResult<CommerceTranscationsData> {
        todo!()
    }

    into_builder!(current, CommerceTranscationsCurrentBuilder, Self);
    into_builder!(history, CommerceTranscationsHistoryBuilder, Self);
}

#[derive(Deserialize, Serialize)]
pub struct CommerceTranscationsCurrentData {}

#[derive(Clone)]
pub struct CommerceTranscationsCurrentBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl CommerceTranscationsCurrentBuilder {
    new_builder_from_params!();

    pub async fn build(self) -> ApiResult<CommerceTranscationsCurrentData> {
        todo!()
    }

    into_builder!(buys, CommerceTranscationsCurrentBuysBuilder, Self);
    into_builder!(sells, CommerceTranscationsCurrentSellsBuilder, Self);
}

#[derive(Deserialize, Serialize)]
pub struct CommerceTranscationsHistoryData {}

#[derive(Clone)]
pub struct CommerceTranscationsHistoryBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl CommerceTranscationsHistoryBuilder {
    new_builder_from_params!();

    pub async fn build(self) -> ApiResult<CommerceTranscationsHistoryData> {
        todo!()
    }

    into_builder!(buys, CommerceTranscationsHistoryBuysBuilder, Self);
    into_builder!(sells, CommerceTranscationsHistorySellsBuilder, Self);
}

#[derive(Deserialize, Serialize)]
pub struct CommerceTranscationsCurrentBuysData {}

pub struct CommerceTranscationsCurrentBuysBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl CommerceTranscationsCurrentBuysBuilder {
    new_builder_from_params!();

    pub async fn build(self) -> ApiResult<CommerceTranscationsCurrentBuysData> {
        todo!()
    }
}

#[derive(Deserialize, Serialize)]
pub struct CommerceTranscationsCurrentSellsData {}

pub struct CommerceTranscationsCurrentSellsBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl CommerceTranscationsCurrentSellsBuilder {
    new_builder_from_params!();

    pub async fn build(self) -> ApiResult<CommerceTranscationsCurrentSellsData> {
        todo!()
    }
}

#[derive(Deserialize, Serialize)]
pub struct CommerceTranscationsHistoryBuysData {}

pub struct CommerceTranscationsHistoryBuysBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl CommerceTranscationsHistoryBuysBuilder {
    new_builder_from_params!();

    pub async fn build(self) -> ApiResult<CommerceTranscationsHistoryBuysData> {
        todo!()
    }
}

#[derive(Deserialize, Serialize)]
pub struct CommerceTranscationsHistorySellsData {}

pub struct CommerceTranscationsHistorySellsBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl CommerceTranscationsHistorySellsBuilder {
    new_builder_from_params!();

    pub async fn build(self) -> ApiResult<CommerceTranscationsHistorySellsData> {
        todo!()
    }
}
