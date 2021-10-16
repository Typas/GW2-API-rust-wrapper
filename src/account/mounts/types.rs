use crate::api::NotAuthenticatedError;
use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct AccountTypesData {}

impl AccountTypesData {}

pub struct AccountTypesBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl AccountTypesBuilder {
    new_builder_from_params!();

    pub async fn build(self) -> ApiResult<AccountTypesData> {
        todo!()
    }
}
