use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct AchievementsGroupsData {}

impl AchievementsGroupsData {}

#[derive(Clone)]
pub struct AchievementsGroupsBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl AchievementsGroupsBuilder {
    new_builder_from_params!();

    pub async fn build(self) -> ApiResult<AchievementsGroupsData> {
        todo!()
    }

    into_builder!(id, AchievementsGroupsIdBuilder, guid: String);
    into_builder!(ids, AchievementsGroupsMultiIdBuilder, guids: Vec<String>);
}

#[derive(Deserialize, Serialize)]
pub struct AchievementsGroupsIdData {}

pub struct AchievementsGroupsIdBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    guid: String,
}

impl AchievementsGroupsIdBuilder {
    new_builder_from_params!(guid: String);

    pub async fn build(self) -> ApiResult<AchievementsGroupsIdData> {
        todo!()
    }
}

pub struct AchievementsGroupsMultiIdBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    guids: Vec<String>,
}

impl AchievementsGroupsMultiIdBuilder {
    new_builder_from_params!(guids: Vec<String>);

    pub async fn build(self) -> ApiResult<Vec<AchievementsGroupsIdData>> {
        todo!()
    }
}
