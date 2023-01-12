pub mod achievements;
pub mod bank;
pub mod dailycrafting;
pub mod dungeons;
pub mod dyes;
pub mod finishers;
pub mod gliders;
pub mod home;
pub mod inventory;
pub mod legendaryarmory;
pub mod luck;
pub mod mailcarriers;
pub mod mapchests;
pub mod masteries;
pub mod mastery;
pub mod materials;
pub mod minis;
pub mod mounts;
pub mod novelties;
pub mod outfits;
pub mod pvp;
pub mod raids;
pub mod recipes;
pub mod skins;
pub mod titles;
pub mod wallet;
pub mod worldbosses;

use crate::api::NotAuthenticatedError;
use crate::util::*;
use crate::{ApiClient, ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct Data {
    id: String,
    age: u32,
    name: String,
    world: u32,
    guilds: Vec<String>,
    guild_leader: Option<Vec<String>>,
    created: String,
    access: Vec<String>,
    commander: bool,
    fractal_level: Option<u32>,
    daily_ap: Option<u32>,
    monthly_ap: Option<u32>,
    wvw_rank: Option<u32>,
    last_modified: Option<String>,
    build_storage_slots: Option<u32>,
}

impl Data {
    /// The unique persistent account GUID.
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    /// The age of the account in seconds.
    pub fn age(&self) -> u32 {
        self.age
    }

    /// The unique acount name with numerical suffix.
    /// *It is possible that the name change.*
    /// Do not rely on the name, use `id` instead.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// The id of the home world the account is assigned to.
    /// Can be resolved against `/v2/worlds`.
    pub fn world(&self) -> u32 {
        self.world
    }

    /// A list of guilds to the given account.
    pub fn guilds(&self) -> Vec<String> {
        self.guilds.clone()
    }

    /// A list of guilds the account is leader of.
    /// Requires the additional `guilds` scope.
    pub fn guild_leader(&self) -> Option<Vec<String>> {
        self.guild_leader.clone()
    }

    /// An ISO-8601 standard timestamp of when the account was created.
    pub fn created(&self) -> &str {
        self.created.as_str()
    }

    /// A list of what content this account has access to.
    /// Possible values:
    /// - `None` - should probably never happen
    /// - `PlayForFree` - has not yet purchased the game
    /// - `GuildWars2` - has purchased the base game
    /// - `HeartOfThorns` - has purchased Heart of Thorns
    /// - `PathOfFire` - has purchased Path of Fire
    pub fn access(&self) -> Vec<String> {
        self.access.clone()
    }

    /// True if the player has bought a commander tag.
    pub fn commander(&self) -> bool {
        self.commander
    }

    /// The account's personal fractal reward level.
    /// Requires the additional `progression` scope
    pub fn fractal_level(&self) -> Option<u32> {
        self.fractal_level
    }

    /// The daily AP the account has.
    /// Requires the additional `progression` scope.
    pub fn daily_ap(&self) -> Option<u32> {
        self.daily_ap
    }

    /// The monthly AP the account has.
    /// Requires the additional `progression` scope.
    pub fn monthly_ap(&self) -> Option<u32> {
        self.monthly_ap
    }

    /// The account's personal wvw rank.
    /// Requires the additional `progression` scope.
    pub fn wvw_rank(&self) -> Option<u32> {
        self.wvw_rank
    }

    /// An ISO-8601 standard timestamp of when the account information last changed as perceived by the API.
    /// This field is only present when a Schema version of `2019-02-21T00:00:00Z` or later is requested.
    pub fn last_modified(&self) -> Option<String> {
        self.last_modified.clone()
    }

    /// The amount of build storage slot an account has unlocked. Requires additional `builds` scope.
    /// This field is only present when a Schema version of `2019-12-19T00:00:00.000Z` or later is requested.
    pub fn build_storage_slots(&self) -> Option<u32> {
        self.build_storage_slots
    }
}

#[derive(Clone)]
pub struct Builder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
}

impl Builder {
    /// Get account data from api.guildwars2.com .
    /// Returns `NotAuthenticatedError` when trying to access without api key
    pub async fn get(self) -> ApiResult<Data> {
        if let None = Option::as_ref(&self.key) {
            return Err(Box::new(NotAuthenticatedError));
        }

        let req = request_common_build(&self.client, &self.key, &self.version, &self.url);

        let data = req.send().await?.json().await?;
        Ok(data)
    }

    into_builder!(home, home::Builder);
    into_builder!(mastery, mastery::Builder);
    into_builder!(mounts, mounts::Builder);
    into_builder!(pvp, pvp::Builder);
    into_builder!(achievements, achievements::Builder);
    into_builder!(bank, bank::Builder);
    into_builder!(dailycrafting, dailycrafting::Builder);
    into_builder!(dungeons, dungeons::Builder);
    into_builder!(dyes, dyes::Builder);
    into_builder!(finishers, finishers::Builder);
    into_builder!(gliders, gliders::Builder);
    into_builder!(inventory, inventory::Builder);
    into_builder!(luck, luck::Builder);
    into_builder!(legendaryarmory, legendaryarmory::Builder);
    into_builder!(mailcarriers, mailcarriers::Builder);
    into_builder!(mapchests, mapchests::Builder);
    into_builder!(masteries, masteries::Builder);
    into_builder!(materials, materials::Builder);
    into_builder!(minis, minis::Builder);
    into_builder!(novelties, novelties::Builder);
    into_builder!(outfits, outfits::Builder);
    into_builder!(raids, raids::Builder);
    into_builder!(recipes, recipes::Builder);
    into_builder!(skins, skins::Builder);
    into_builder!(titles, titles::Builder);
    into_builder!(wallet, wallet::Builder);
    into_builder!(worldbosses, worldbosses::Builder);
}

impl From<&ApiClient> for Builder {
    fn from(source: &ApiClient) -> Self {
        Self {
            client: source.client.clone(),
            key: source.key.clone(),
            version: source.version.clone(),
            url: "https://api.guildwars2.com/v2/account".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ApiClient;

    fn setup() -> ApiClient {
        ApiClient::new().unwrap()
    }

    // need environment variable GW2_API_KEY
    fn setup_auth() -> ApiClient {
        let key = std::env::var("GW2_API_KEY").unwrap();
        ApiClient::config().unwrap().key(&key).build()
    }

    #[tokio::test]
    #[should_panic]
    async fn build_no_auth() {
        let client = setup();
        let _t: super::Data = client.account().build().await.unwrap();
    }

    #[tokio::test]
    async fn build_with_auth() {
        let client = setup_auth();
        let _t: super::Data = client.account().build().await.unwrap();
    }
}
