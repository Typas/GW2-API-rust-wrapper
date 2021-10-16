use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct BackStoryQuestionsData {}

impl BackStoryQuestionsData {}

#[derive(Clone)]
pub struct BackStoryQuestionsBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl BackStoryQuestionsBuilder {
    new_builder_from_params!();

    pub async fn build(self) -> ApiResult<BackStoryQuestionsData> {
        todo!()
    }

    into_builder!(id, BackStoryQuestionsIdBuilder, sid: String);
    into_builder!(ids, BackStoryQuestionsMultiIdBuilder, sids: Vec<String>);
}

#[derive(Deserialize, Serialize)]
pub struct BackStoryQuestionsIdData {}

pub struct BackStoryQuestionsIdBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    sid: String,
}

impl BackStoryQuestionsIdBuilder {
    new_builder_from_params!(sid: String);

    pub async fn build(self) -> ApiResult<BackStoryQuestionsIdData> {
        todo!()
    }
}

pub struct BackStoryQuestionsMultiIdBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    sids: Vec<String>,
}

impl BackStoryQuestionsMultiIdBuilder {
    new_builder_from_params!(sids: Vec<String>);

    pub async fn build(self) -> ApiResult<BackStoryQuestionsIdData> {
        todo!()
    }
}
