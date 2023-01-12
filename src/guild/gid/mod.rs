pub mod log;
pub mod members;
pub mod ranks;
pub mod stash;
pub mod storage;
pub mod teams;
pub mod treasury;
pub mod upgrades;

use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct Data {}

impl Data {}

#[derive(Clone)]
pub struct Builder {
    pub(super) client: Client,
    pub(super) key: Arc<Option<String>>,
    pub(super) version: Arc<SchemaVersion>,
    pub(super) url: String,
}

impl Builder {
    pub async fn get(self) -> ApiResult<Data> {
        todo!()
    }

    into_builder!(log, log::Builder);
    into_builder!(members, members::Builder);
    into_builder!(ranks, ranks::Builder);
    into_builder!(stash, stash::Builder);
    into_builder!(storage, storage::Builder);
    into_builder!(teams, teams::Builder);
    into_builder!(treasury, treasury::Builder);
    into_builder!(upgrades, upgrades::Builder);
}
