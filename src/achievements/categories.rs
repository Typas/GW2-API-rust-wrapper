use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct AchievementsCategoriesData {}

impl AchievementsCategoriesData {}

#[derive(Clone)]
pub struct AchievementsCategoriesBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl AchievementsCategoriesBuilder {
    new_builder_from_params!();

    pub async fn build(self) -> ApiResult<AchievementsCategoriesData> {
        todo!()
    }

    into_builder!(id, AchievementsCategoriesIdBuilder, id: u32);
    into_builder!(ids, AchievementsCategoriesMultiIdBuilder, ids: Vec<u32>);
}

pub struct AchievementsCategoriesIdData {}

impl AchievementsCategoriesIdData {}

pub struct AchievementsCategoriesIdBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    id: u32,
}

impl AchievementsCategoriesIdBuilder {
    new_builder_from_params!(id: u32);

    pub async fn build(self) -> ApiResult<AchievementsCategoriesIdData> {
        todo!()
    }
}

pub struct AchievementsCategoriesMultiIdBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    ids: Vec<u32>,
}

impl AchievementsCategoriesMultiIdBuilder {
    new_builder_from_params!(ids: Vec<u32>);

    pub async fn build(self) -> ApiResult<Vec<AchievementsCategoriesIdData>> {
        todo!()
    }
}
