use super::{ApiResult, SchemaVersion};
use crate::api::NotAuthenticatedError;
use crate::util::request_common_build;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct AccountLuckData {}

impl AccountLuckData {}

pub struct AccountLuckBuilder {
    pub client: Client,
    pub key: Arc<Option<String>>,
    pub version: Arc<SchemaVersion>,
}

impl AccountLuckBuilder {
    pub async fn build(self) -> ApiResult<AccountLuckData> {
        todo!()
    }
}
