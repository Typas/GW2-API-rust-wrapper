use super::{ApiResult, SchemaVersion};
use crate::api::NotAuthenticatedError;
use crate::util::request_common_build;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct AccountMapChestsData {}

impl AccountMapChestsData {}

pub struct AccountMapChestsBuilder {
    pub client: Client,
    pub key: Arc<Option<String>>,
    pub version: Arc<SchemaVersion>,
}

impl AccountMapChestsBuilder {
    pub async fn build(self) -> ApiResult<AccountMapChestsData> {
        todo!()
    }
}
