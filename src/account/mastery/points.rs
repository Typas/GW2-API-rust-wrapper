use crate::{ApiResult, SchemaVersion};
use crate::api::NotAuthenticatedError;
use crate::util::request_common_build;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct AccountMasteryPointsData {}

impl AccountMasteryPointsData {}

pub struct AccountMasteryPointsBuilder {
    pub client: Client,
    pub key: Arc<Option<String>>,
    pub version: Arc<SchemaVersion>,
}

impl AccountMasteryPointsBuilder {
    pub async fn build(self) -> ApiResult<AccountMasteryPointsData> {
        todo!()
    }
}
