pub mod answers;
pub mod questions;

use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct BackStoryData {}

#[derive(Clone)]
pub struct BackStoryBuilder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
}

impl BackStoryBuilder {
    new_builder_from_params!();

    pub async fn build(self) -> ApiResult<BackStoryData> {
        todo!()
    }

    into_builder!(answers, BackStoryAnswersBuilder);
    into_builder!(questions, BackStoryQuestionsBuilder);
}
