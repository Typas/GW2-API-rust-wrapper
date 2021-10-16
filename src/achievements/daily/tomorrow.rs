use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub enum AchievementsDailyTomorrowData {
    Pre20190516(AchievementDailyTomorrowDataPre20190516Schema),
    Post20190516(AchievementDailyTomorrowDataPost20190516Schema),
}

impl AchievementsDailyTomorrowData {}

#[derive(Deserialize, Serialize)]
pub struct AchievementDailyTomorrowDataPre20190516Schema {}

trait_from_jsvalue!(AchievementDailyTomorrowDataPre20190516Schema);

#[derive(Deserialize, Serialize)]
pub struct AchievementDailyTomorrowDataPost20190516Schema {}

trait_from_jsvalue!(AchievementDailyTomorrowDataPost20190516Schema);

pub struct AchievementsDailyTomorrowBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl AchievementsDailyTomorrowBuilder {
    new_builder_from_params!();

    pub async fn build(self) -> ApiResult<AchievementsDailyTomorrowData> {
        todo!()
    }
}
