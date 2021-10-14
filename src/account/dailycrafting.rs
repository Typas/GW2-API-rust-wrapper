use super::{ApiResult, SchemaVersion};
use crate::api::NotAuthenticatedError;
use crate::util::request_common_build;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct AccountDailyCraftingData {}

impl AccountDailyCraftingData {}

pub struct AccountDailyCraftingBuilder {
    pub client: Client,
    pub key: Arc<Option<String>>,
    pub version: Arc<SchemaVersion>,
}

impl AccountDailyCraftingBuilder {
    pub async fn build(self) -> ApiResult<AccountDailyCraftingData> {
        todo!()
    }
}
