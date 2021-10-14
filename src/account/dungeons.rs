use super::{ApiResult, SchemaVersion};
use crate::api::NotAuthenticatedError;
use crate::util::request_common_build;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct AccountDungeonsData {}

impl AccountDungeonsData {}

pub struct AccountDungeonsBuilder {
    pub client: Client,
    pub key: Arc<Option<String>>,
    pub version: Arc<SchemaVersion>,
}

impl AccountDungeonsBuilder {
    pub async fn build(self) -> ApiResult<AccountDungeonsData> {
        todo!()
    }
}
