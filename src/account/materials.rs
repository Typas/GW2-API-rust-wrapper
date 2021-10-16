use super::{ApiResult, SchemaVersion};
use crate::api::NotAuthenticatedError;
use crate::util::request_common_build;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct AccountMaterialsData {}

impl AccountMaterialsData {}

pub struct AccountMaterialsBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl AccountMaterialsBuilder {
    pub async fn build(self) -> ApiResult<AccountMaterialsData> {
        todo!()
    }
}
