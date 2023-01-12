use crate::api::NotAuthenticatedError;
use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use serde_json::Value as JsonValue;

#[derive(Deserialize, Serialize)]
pub struct Data {
    achievements: Vec<Achievement>,
}

impl Data {
    pub fn from_json_value(json: JsonValue) -> ApiResult<Self> {
        let data: Vec<Achievement> = serde_json::from_value(json)?;
        let data = Self { achievements: data };

        Ok(data)
    }
}

impl std::ops::Deref for Data {
    type Target = Vec<Achievement>;
    fn deref(&self) -> &Self::Target {
        &self.achievements
    }
}


#[derive(Deserialize, Serialize)]
pub struct Achievement {
    id: u32,
    bits: Option<Vec<u32>>,
    current: Option<u32>,
    max: Option<u32>,
    done: bool,
    repeated: Option<u32>,
    unlocked: Option<bool>,
}

impl Achievement {
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

trait_try_from_jsonvalue!(Achievement);

pub struct Builder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
}

impl Builder {

    pub async fn get(self) -> ApiResult<Data> {
        if let None = Option::as_ref(&self.key) {
            return Err(Box::new(NotAuthenticatedError));
        }
        let req = request_common_build(&self.client, &self.key, &self.version, &self.url);

        // XXX: inconsistency of store into data
        let data: Vec<Achievement> = req.send().await?.json().await?;
        Ok(Data { achievements: data })
    }
}

impl From<super::Builder> for Builder {
    fn from(source: super::Builder) -> Self {
        Self {
            client: source.client,
            key: source.key,
            version: source.version,
            url: source.url + "/achievements",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ApiClient;

    fn setup() -> ApiClient {
        ApiClient::new().unwrap()
    }

    // need environment variable GW2_API_KEY
    fn setup_auth() -> ApiClient {
        let key = std::env::var("GW2_API_KEY").unwrap();
        ApiClient::config().unwrap().key(&key).build()
    }

    #[tokio::test]
    #[should_panic]
    async fn build_no_auth() {
        let client = setup();
        let _t: super::Data = client.account().achievements().build().await.unwrap();
    }

    #[tokio::test]
    async fn build_with_auth() {
        let client = setup_auth();
        let _t: super::Data = client.account().achievements().build().await.unwrap();
    }
}
