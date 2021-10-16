pub mod categories;
pub mod daily;
pub mod groups;

use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct AchievementsData {}

#[derive(Clone)]
pub struct AchievementsBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl AchievementsBuilder {
    new_builder_from_params!();

    pub async fn build(self) -> ApiResult<AchievementsData> {
        let url = "https://api.guildwars2.com/v2/achievements";

        let req = self.client.get(url);
        let req = request_common_build(req, &self.key, &self.version);

        let data = req.send().await?.json().await?;
        Ok(data)
    }

    into_builder!(daily, AchievementsDailyBuilder);
    into_builder!(categories, AchievementsCategoriesBuilder);
    into_builder!(groups, AchievementsGroupsBuilder);

    into_builder!(id, AchievementsIdBuilder, id: u32);
    into_builder!(ids, AchievementsMultiIdBuilder, ids: Vec<u32>);
}

#[derive(Deserialize, Serialize)]
pub struct AchievementsIdData {}

pub struct AchievementsIdBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    id: u32,
}

impl AchievementsIdBuilder {
    new_builder_from_params!(id: u32);

    pub async fn build(self) -> ApiResult<AchievementsIdData> {
        todo!()
    }
}

pub struct AchievementsMultiIdBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    ids: Vec<u32>,
}

impl AchievementsMultiIdBuilder {
    new_builder_from_params!(ids: Vec<u32>);

    pub async fn build(self) -> ApiResult<Vec<AchievementsIdData>> {
        todo!()
    }
}
