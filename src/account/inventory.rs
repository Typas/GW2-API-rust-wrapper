use super::{ApiResult, SchemaVersion};
use crate::api::NotAuthenticatedError;
use crate::util::request_common_build;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct AccountInventoryData {}

impl AccountInventoryData {}

pub struct AccountInventoryBuilder {
    pub client: Client,
    pub key: Arc<Option<String>>,
    pub version: Arc<SchemaVersion>,
}

impl AccountInventoryBuilder {
    pub async fn build(self) -> ApiResult<AccountInventoryData> {
        todo!()
    }
}
