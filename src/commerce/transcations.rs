use crate::api::NotAuthenticatedError;
use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct Data {}

impl Data {}

#[derive(Clone)]
pub struct Builder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
}

impl Builder {
    pub async fn get(self) -> ApiResult<Data> {
        todo!()
    }

    into_builder!(current, CurrentBuilder);
    into_builder!(history, HistoryBuilder);
}

impl From<super::Builder> for Builder {
    fn from(source: super::Builder) -> Self {
        Self {
            client: source.client,
            key: source.key,
            version: source.version,
            url: source.url + "/transcations",
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CurrentData {}

impl CurrentData {}

#[derive(Clone)]
pub struct CurrentBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
}

impl CurrentBuilder {
    pub async fn get(self) -> ApiResult<CurrentData> {
        todo!()
    }

    into_builder!(buys, CurrentBuysBuilder);
    into_builder!(sells, CurrentSellsBuilder);
}

impl From<Builder> for CurrentBuilder {
    fn from(source: Builder) -> Self {
        Self {
            client: source.client,
            key: source.key,
            version: source.version,
            url: source.url + "/current",
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct HistoryData {}

impl HistoryData {}

#[derive(Clone)]
pub struct HistoryBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
}

impl HistoryBuilder {
    pub async fn get(self) -> ApiResult<HistoryData> {
        todo!()
    }

    into_builder!(buys, HistoryBuysBuilder);
    into_builder!(sells, HistorySellsBuilder);
}

impl From<Builder> for HistoryBuilder {
    fn from(source: Builder) -> Self {
        Self {
            client: source.client,
            key: source.key,
            version: source.version,
            url: source.url + "/history",
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CurrentBuysData {}

impl CurrentBuysData {}

pub struct CurrentBuysBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
}

impl CurrentBuysBuilder {
    pub async fn get(self) -> ApiResult<CurrentBuysData> {
        todo!()
    }
}

impl From<CurrentBuilder> for CurrentBuysBuilder {
    fn from(source: CurrentBuilder) -> Self {
        Self {
            client: source.client,
            key: source.key,
            version: source.version,
            url: source.url + "/buys",
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CurrentSellsData {}

pub struct CurrentSellsBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
}

impl CurrentSellsBuilder {
    pub async fn get(self) -> ApiResult<CurrentSellsData> {
        todo!()
    }
}

impl From<CurrentBuilder> for CurrentSellsBuilder {
    fn from(source: CurrentBuilder) -> Self {
        Self {
            client: source.client,
            key: source.key,
            version: source.version,
            url: source.url + "/sells",
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct HistoryBuysData {}

impl HistoryBuysData {}

pub struct HistoryBuysBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
}

impl HistoryBuysBuilder {
    pub async fn get(self) -> ApiResult<HistoryBuysData> {
        todo!()
    }
}

impl From<HistoryBuilder> for HistoryBuysBuilder {
    fn from(source: HistoryBuilder) -> Self {
        Self {
            client: source.client,
            key: source.key,
            version: source.version,
            url: source.url + "/buys",
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct HistorySellsData {}

pub struct HistorySellsBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
}

impl HistorySellsBuilder {
    pub async fn get(self) -> ApiResult<HistorySellsData> {
        todo!()
    }
}

impl From<HistoryBuilder> for HistorySellsBuilder {
    fn from(source: HistoryBuilder) -> Self {
        Self {
            client: source.client,
            key: source.key,
            version: source.version,
            url: source.url + "/sells",
        }
    }
}
