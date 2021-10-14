use crate::{ApiResult, SchemaVersion};
use crate::api::NotAuthenticatedError;
use crate::util::request_common_build;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct AccountSkinsData {}

impl AccountSkinsData {}

pub struct AccountSkinsBuilder {
    pub client: Client,
    pub key: Arc<Option<String>>,
    pub version: Arc<SchemaVersion>,
}

impl AccountSkinsBuilder {
    pub async fn build(self) -> ApiResult<AccountSkinsData> {
        todo!()
    }
}
