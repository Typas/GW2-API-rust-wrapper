pub mod achievements;
pub mod bank;
pub mod dailycrafting;
pub mod dungeons;
pub mod dyes;
pub mod finishers;
pub mod gliders;
pub mod home;
pub mod inventory;
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

use super::{ApiResult, SchemaVersion};
use crate::api::NotAuthenticatedError;
use crate::util::{request_common_build, to_builder};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct AccountData {
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
}

impl AccountData {
    pub fn new(json: serde_json::Value) -> ApiResult<Self> {
        let data: Self = serde_json::from_value(json)?;

        Ok(data)
    }

    /// The unique persistent account GUID.
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    // /// The age of the account,
    // pub fn age(&self) -> String {
    //     let days = self.age / 86400;
    //     let hours = self.age % 86400 / 3600;
    //     let minutes = self.age % 3600 / 60;
    //     let seconds = self.age % 60;

    //     format!("{} d {} h {} m {} s", days, hours, minutes, seconds).to_string()
    // }

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
}

#[derive(Clone)]
pub struct AccountBuilder {
    pub client: Client,
    pub key: Arc<Option<String>>,
    pub version: Arc<SchemaVersion>,
}

impl AccountBuilder {
    /// Get account data from api.guildwars2.com .
    /// Returns `NotAuthenticatedError` when trying to access without api key
    pub async fn build(self) -> ApiResult<AccountData> {
        if let None = Option::as_ref(&self.key) {
            return Err(Box::new(NotAuthenticatedError));
        }
        let url = "https://api.guildwars2.com/v2/account";

        let req = self.client.get(url);
        let req = request_common_build(req, &self.key, &self.version);

        let data = req.send().await?.json().await?;
        Ok(data)
    }

    to_builder!(home, AccountHomeBuilder);
    to_builder!(mastery, AccountMasteryBuilder);
    to_builder!(mounts, AccountMountsBuilder);
    to_builder!(pvp, AccountPvpBuilder);
    to_builder!(achievements, AccountAchievementsBuilder);
    to_builder!(bank, AccountBankBuilder);
    to_builder!(dailycrafting, AccountDailyCraftingBuilder);
    to_builder!(dungeons, AccountDungeonsBuilder);
    to_builder!(dyes, AccountDyesBuilder);
    to_builder!(finishers, AccountFinishersBuilder);
    to_builder!(gliders, AccountGlidersBuilder);
    to_builder!(inventory, AccountInventoryBuilder);
    to_builder!(luck, AccountLuckBuilder);
    to_builder!(mailcarriers, AccountMailCarriersBuilder);
    to_builder!(mapchests, AccountMapChestsBuilder);
    to_builder!(masteries, AccountMasteriesBuilder);
    to_builder!(minis, AccountMinisBuilder);
    to_builder!(novelties, AccountNoveltiesBuilder);
    to_builder!(outfits, AccountOutfitsBuilder);
    to_builder!(raids, AccountRaidsBuilder);
    to_builder!(recipes, AccountRecipesBuilder);
    to_builder!(skins, AccountSkinsBuilder);
    to_builder!(titles, AccountTitlesBuilder);
    to_builder!(wallet, AccountWalletBuilder);
    to_builder!(worldbosses, AccountWorldBossesBuilder);
}
