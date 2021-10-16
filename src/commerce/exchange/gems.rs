use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct CommerceExchangeGemsData {}

impl CommerceExchangeGemsData {}

pub struct CommerceExchangeGemsBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl CommerceExchangeGemsBuilder {
    new_builder_from_params!();

    pub async fn build(self) -> ApiResult<CommerceExchangeGemsData> {
        self.build_with_quantity(400).await
    }

    pub async fn build_with_quantity(self, _quantity: u32) -> ApiResult<CommerceExchangeGemsData> {
        todo!()
    }
}
