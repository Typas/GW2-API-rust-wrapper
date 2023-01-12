pub mod tomorrow;

use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::sync::Arc;

pub enum Data {
    Pre20190516(AchievementsDailyDataPre20190516Schema),
    Post20190516(AchievementsDailyDataPost20190516Schema),
}

#[derive(Deserialize, Serialize)]
pub struct AchievementsDailyDataPre20190516Schema {}

trait_try_from_jsonvalue!(AchievementsDailyDataPre20190516Schema);

#[derive(Deserialize, Serialize)]
pub struct AchievementsDailyDataPost20190516Schema {}

trait_try_from_jsonvalue!(AchievementsDailyDataPost20190516Schema);

#[derive(Clone)]
pub struct Builder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
}

impl Builder {
    pub async fn get(self) -> ApiResult<Data> {
        let req = request_common_build(&self.client, &self.key, &self.version, &self.url);

        let tmp: JsonValue = req.send().await?.json().await?;

        let data = match *self.version {
            SchemaVersion::Default => Data::Pre20190516(tmp.try_into()?),
            SchemaVersion::Time(t) => {
                let t = DateTime::<Utc>::from_utc(t, Utc);
                let crit_datetime =
                    DateTime::parse_from_rfc3339("2019-05-16T00:00:00-00:00").unwrap();
                if t < crit_datetime {
                    Data::Pre20190516(tmp.try_into()?)
                } else {
                    Data::Post20190516(tmp.try_into()?)
                }
            }
            SchemaVersion::Latest => Data::Post20190516(tmp.try_into()?),
        };

        Ok(data)
    }

    into_builder!(tomorrow, tomorrow::Builder);
}

impl From<super::Builder> for Builder {
    fn from(source: super::Builder) -> Self {
        Self {
            client: source.client,
            key: source.key,
            version: source.version,
            url: source.url + "/daily",
        }
    }
}
