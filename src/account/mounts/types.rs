use crate::{ApiResult, SchemaVersion};
use crate::api::NotAuthenticatedError;
use crate::util::request_common_build;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct AccountTypesData {}

impl AccountTypesData {}

pub struct AccountTypesBuilder {
    pub client: Client,
    pub key: Arc<Option<String>>,
    pub version: Arc<SchemaVersion>,
}

impl AccountTypesBuilder {
    pub async fn build(self) -> ApiResult<AccountTypesData> {
        todo!()
    }
}
