// modules
pub mod account;
pub mod achievements;
pub mod api;
pub mod backstory;
pub mod build;
pub mod characters;
pub mod colors;
pub mod commerce;
pub mod continents;
pub mod createsubtoken;
pub mod currencies;
pub mod dailycrafting;
pub mod dungeons;
pub mod emblem;
pub mod files;
pub mod finishers;
pub mod guild;
pub mod home;
pub mod items;
pub mod itemstats;
pub mod legends;
pub mod mapchests;
pub mod maps;
pub mod masteries;
pub mod materials;
pub mod minis;
pub mod mounts;
pub mod novelties;
pub mod outfits;
pub mod pets;
pub mod professions;
pub mod pvp;
pub mod quaggans;
pub mod quests;
pub mod races;
pub mod raids;
pub mod recipes;
pub mod skills;
pub mod skins;
pub mod specializations;
pub mod stories;
pub mod titles;
pub mod tokeninfo;
pub mod traits;
mod util;
pub mod worldbosses;
pub mod worlds;
pub mod wvw;

// re-export
pub use api::ApiClient;
pub use api::EndPoint;
pub use api::SchemaVersion;

// type redefinition
type ApiResult<T> = Result<T, Box<dyn std::error::Error>>;

// enumerations
pub enum Error {
    NotAuthenticated,
}

// trait for my ease
pub trait ApiDataType {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
