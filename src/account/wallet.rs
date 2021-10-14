use super::{ApiResult, SchemaVersion};
use crate::api::NotAuthenticatedError;
use crate::util::request_common_build;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct AccountWalletData {}

impl AccountWalletData {}

pub struct AccountWalletBuilder {
    pub client: Client,
    pub key: Arc<Option<String>>,
    pub version: Arc<SchemaVersion>,
}

impl AccountWalletBuilder {
    pub async fn build(self) -> ApiResult<AccountWalletData> {
        todo!()
    }
}
