use crate::api::NotAuthenticatedError;
use crate::util::*;
use crate::{ApiResult, SchemaVersion};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub struct Data {
    id: u32,
    count: u32,
    charges: Option<u32>,
    skin: Option<u32>,
    dyes: Option<Vec<u32>>,
    upgrades: Option<Vec<u32>>,
    upgrade_slot_indices: Option<Vec<u32>>,
    infusions: Option<Vec<u32>>,
    binding: Option<String>,
    bound_to: Option<String>,
    stats: Option<Stats>,
}

#[derive(Deserialize, Serialize)]
pub struct Stats {
    id: u32,
    attributes: Option<Attributes>,
}

#[derive(Deserialize, Serialize)]
pub struct Attributes {
    agony_resistance: Option<u32>,
    boon_duration: Option<u32>,
    condition_damage: Option<u32>,
    condition_duration: Option<u32>,
    crit_damage: Option<u32>,
    healing: Option<u32>,
    power: Option<u32>,
    precision: Option<u32>,
    toughness: Option<u32>,
    vitality: Option<u32>,
}

impl Data {
    /// The item's ID.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// The amount of items in the stack.
    pub fn count(&self) -> u32 {
        self.count
    }

    /// The amount of charges remaining on the item.
    pub fn charges(&self) -> Option<u32> {
        self.charges
    }

    /// The skin applied to the item, if it is different from its original.
    /// Can be resolved against `/v2/skins`.
    pub fn skin(&self) -> Option<u32> {
        self.skin
    }

    /// The IDs of the dyes applied to the item.
    /// Can be resolved against `/v2/colors`.
    pub fn dyes(&self) -> Option<&Vec<u32>> {
        self.dyes.as_ref()
    }

    /// The item IDs of the runes or sigils applied to the item.
    pub fn upgrades(&self) -> Option<&Vec<u32>> {
        self.upgrades.as_ref()
    }

    /// The slot occupied by the upgrade at the corresponding position in `upgrades`.
    pub fn upgrade_slot_indices(&self) -> Option<&Vec<u32>> {
        self.upgrade_slot_indices.as_ref()
    }

    /// An array of item IDs for each infusion applied to the item.
    pub fn infusions(&self) -> Option<&Vec<u32>> {
        self.infusions.as_ref()
    }

    /// The current binding of the item.
    /// Either `Account` or `Character` if present.
    pub fn binding(&self) -> Option<&String> {
        self.binding.as_ref()
    }

    /// If `binding` is `Character`, this field tells which character it is bound to.
    pub fn bound_to(&self) -> Option<&String> {
        self.bound_to.as_ref()
    }

    /// Test whether item contains stats or not.
    pub fn has_stats(&self) -> bool {
        self.stats.is_some()
    }

    /// The ID of the item's stats.
    /// Can be resolved against `/v2/itemstats`.
    pub fn stats_id(&self) -> Option<u32> {
        self.has_stats().then_some(self.stats.as_ref().unwrap().id)
    }

    /// The stats provided by this item.
    pub fn attributes(&self) -> Option<&Attributes> {
        self.has_stats()
            .then_some(self.stats.as_ref().unwrap().attributes.as_ref().unwrap())
    }
}

impl Attributes {
    pub fn agony_resistance(&self) -> Option<u32> {
        self.agony_resistance
    }

    pub fn boon_duration(&self) -> Option<u32> {
        self.boon_duration
    }

    pub fn condition_damage(&self) -> Option<u32> {
        self.condition_damage
    }

    pub fn condition_duration(&self) -> Option<u32> {
        self.condition_duration
    }

    pub fn crit_damage(&self) -> Option<u32> {
        self.crit_damage
    }

    pub fn healing(&self) -> Option<u32> {
        self.healing
    }

    pub fn power(&self) -> Option<u32> {
        self.power
    }

    pub fn precision(&self) -> Option<u32> {
        self.precision
    }

    pub fn toughness(&self) -> Option<u32> {
        self.toughness
    }

    pub fn vitality(&self) -> Option<u32> {
        self.vitality
    }

    pub fn concentration(&self) -> Option<u32> {
        self.boon_duration()
    }

    pub fn expertise(&self) -> Option<u32> {
        self.condition_duration()
    }

    pub fn ferocity(&self) -> Option<u32> {
        self.crit_damage()
    }

    pub fn healing_power(&self) -> Option<u32> {
        self.healing()
    }
}

trait_try_from_jsonvalue!(Data);

pub struct Builder {
    client: Client,
    key: Arc<Option<String>>,
    version: Arc<SchemaVersion>,
    url: String,
}

impl Builder {
    pub async fn get(self) -> ApiResult<Data> {
        todo!()
    }
}

impl From<super::Builder> for Builder {
    fn from(source: super::Builder) -> Self {
        Self {
            client: source.client,
            key: source.key,
            version: source.version,
            url: source.url + "/bank",
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
        let _t: super::Data = client.account().bank().build().await.unwrap();
    }

    #[tokio::test]
    async fn build_with_auth() {
        let client = setup_auth();
        let _t: super::Data = client.account().bank().build().await.unwrap();
    }
}
