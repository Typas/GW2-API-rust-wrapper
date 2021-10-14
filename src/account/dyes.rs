use super::{ApiResult, SchemaVersion};
use crate::api::NotAuthenticatedError;
use crate::util::request_common_build;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct AccountDyesData {}

impl AccountDyesData {}

pub struct AccountDyesBuilder {
    pub client: Client,
    pub key: Arc<Option<String>>,
    pub version: Arc<SchemaVersion>,
}

impl AccountDyesBuilder {
    pub async fn build(self) -> ApiResult<AccountDyesData> {
        todo!()
    }
}
