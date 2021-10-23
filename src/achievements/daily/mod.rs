pub mod tomorrow;

use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsValue;
use std::sync::Arc;

pub enum AchievementsDailyData {
    Pre20190516(AchievementsDailyDataPre20190516Schema),
    Post20190516(AchievementsDailyDataPost20190516Schema),
}

#[derive(Deserialize, Serialize)]
pub struct AchievementsDailyDataPre20190516Schema {}

trait_from_jsvalue!(AchievementsDailyDataPre20190516Schema);

#[derive(Deserialize, Serialize)]
pub struct AchievementsDailyDataPost20190516Schema {}

trait_from_jsvalue!(AchievementsDailyDataPost20190516Schema);

#[derive(Clone)]
pub struct AchievementsDailyBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl AchievementsDailyBuilder {
    new_builder_from_params!();

    pub async fn build(self) -> ApiResult<AchievementsDailyData> {
        let url = "https://api.guildwars2.com/v2/achievements/daily";

        let req = self.client.get(url);
        let req = request_common_build(req, &self.key, &self.version);

        let tmp: JsValue = req.send().await?.json().await?;

        let data = match *self.version {
            SchemaVersion::Default => AchievementsDailyData::Pre20190516(tmp.into()),
            SchemaVersion::Time(t) => {
                let t = DateTime::<Utc>::from_utc(t, Utc);
                let crit_datetime =
                    DateTime::parse_from_rfc3339("2019-05-16T00:00:00-00:00").unwrap();
                if t < crit_datetime {
                    AchievementsDailyData::Pre20190516(tmp.into())
                } else {
                    AchievementsDailyData::Post20190516(tmp.into())
                }
            }
            SchemaVersion::Latest => AchievementsDailyData::Post20190516(tmp.into()),
        };

        Ok(data)
    }

    into_builder!(tomorrow, AchievementsDailyTomorrowBuilder);
}
