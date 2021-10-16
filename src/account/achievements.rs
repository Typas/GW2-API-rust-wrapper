use crate::api::NotAuthenticatedError;
use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct AccountAchievementsData {
    achievements: Vec<SingleAchievement>,
}

impl AccountAchievementsData {
    pub fn new(json: serde_json::Value) -> ApiResult<Self> {
        let data: Vec<SingleAchievement> = serde_json::from_value(json)?;
        let data = Self { achievements: data };

        Ok(data)
    }

    pub fn achievements(&self) -> &Vec<SingleAchievement> {
        &self.achievements
    }
}

#[derive(Deserialize, Serialize)]
pub struct SingleAchievement {
    id: u32,
    bits: Option<Vec<u32>>,
    current: Option<u32>,
    max: Option<u32>,
    done: bool,
    repeated: Option<u32>,
    unlocked: Option<bool>,
}

impl SingleAchievement {
    /// The achievement id.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// This attribute contains an array of numbers,
    /// giving more specific information on the progress for the achievement.
    /// The meaning of each value varies with each achievement.
    /// Bits start at zero.
    /// If an achievement is done, the in-progress bits are not displayed.
    pub fn bits(&self) -> Option<&Vec<u32>> {
        self.bits.as_ref()
    }

    /// The player's current progress towards the achievement.
    pub fn current(&self) -> Option<u32> {
        self.current
    }

    /// The amount needed to complete the achievement.
    pub fn max(&self) -> Option<u32> {
        self.max
    }

    /// Whether or not the achievement is done.
    pub fn done(&self) -> bool {
        self.done
    }

    /// The number of times the achievement has been completed
    /// if the achievement is repeatable.
    pub fn repeated(&self) -> Option<u32> {
        self.repeated
    }

    /// Whether or not the achievement is unlocked.
    pub fn unlocked(&self) -> bool {
        match self.unlocked {
            Some(is_unlocked) => is_unlocked,
            None => true,
        }
    }
}

trait_from_jsvalue!(SingleAchievement);

pub struct AccountAchievementsBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl AccountAchievementsBuilder {
    new_builder_from_params!();

    pub async fn build(self) -> ApiResult<AccountAchievementsData> {
        if let None = Option::as_ref(&self.key) {
            return Err(Box::new(NotAuthenticatedError));
        }
        let url = "https://api.guildwars2.com/v2/account/achievements";

        let req = self.client.get(url);
        let req = request_common_build(req, &self.key, &self.version);

        // XXX: inconsistency of store into data
        let data: Vec<SingleAchievement> = req.send().await?.json().await?;
        Ok(AccountAchievementsData { achievements: data })
    }
}
