use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct BackStoryAnswersData {}

impl BackStoryAnswersData {}

#[derive(Clone)]
pub struct BackStoryAnswersBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl BackStoryAnswersBuilder {
    new_builder_from_params!();

    pub async fn build(self) -> ApiResult<BackStoryAnswersData> {
        todo!()
    }

    into_builder!(id, BackStoryAnswersIdBuilder, sid: String);
    into_builder!(ids, BackStoryAnswersMultiIdBuilder, sids: Vec<String>);
}

#[derive(Deserialize, Serialize)]
pub struct BackStoryAnswersIdData {}

pub struct BackStoryAnswersIdBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    sid: String,
}

impl BackStoryAnswersIdBuilder {
    new_builder_from_params!(sid: String);

    pub async fn build(self) -> ApiResult<BackStoryAnswersIdData> {
        todo!()
    }
}

pub struct BackStoryAnswersMultiIdBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    sids: Vec<String>,
}

impl BackStoryAnswersMultiIdBuilder {
    new_builder_from_params!(sids: Vec<String>);

    pub async fn build(self) -> ApiResult<BackStoryAnswersIdData> {
        todo!()
    }
}
