use super::{ApiResult, SchemaVersion};
use crate::api::NotAuthenticatedError;
use crate::util::request_common_build;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct AccountMailCarriersData {}

impl AccountMailCarriersData {}

pub struct AccountMailCarriersBuilder {
    pub client: Client,
    pub key: Arc<Option<String>>,
    pub version: Arc<SchemaVersion>,
}

impl AccountMailCarriersBuilder {
    pub async fn build(self) -> ApiResult<AccountMailCarriersData> {
        todo!()
    }
}
