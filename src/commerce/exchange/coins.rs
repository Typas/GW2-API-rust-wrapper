use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct CommerceExchangeCoinsData {}

impl CommerceExchangeCoinsData {}

pub struct CommerceExchangeCoinsBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl CommerceExchangeCoinsBuilder {
    new_builder_from_params!();

    pub async fn build(self) -> ApiResult<CommerceExchangeCoinsData> {
        self.build_with_quantity(100_00_00).await
    }

    pub async fn build_with_quantity(self, _quantity: u32) -> ApiResult<CommerceExchangeCoinsData> {
        todo!()
    }
}
