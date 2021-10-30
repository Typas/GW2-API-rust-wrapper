// modules
#[allow(unused_imports, dead_code)]
pub mod account;
#[allow(unused_imports, dead_code)]
pub mod achievements;
#[allow(unused_imports, dead_code)]
pub mod api;
#[allow(unused_imports, dead_code)]
pub mod backstory;
#[allow(unused_imports, dead_code)]
pub mod build;
#[allow(unused_imports, dead_code)]
pub mod characters;
#[allow(unused_imports, dead_code)]
pub mod colors;
#[allow(unused_imports, dead_code)]
pub mod commerce;
#[allow(unused_imports, dead_code)]
pub mod continents;
#[allow(unused_imports, dead_code)]
pub mod createsubtoken;
#[allow(unused_imports, dead_code)]
pub mod currencies;
#[allow(unused_imports, dead_code)]
pub mod dailycrafting;
#[allow(unused_imports, dead_code)]
pub mod dungeons;
#[allow(unused_imports, dead_code)]
pub mod emblem;
#[allow(unused_imports, dead_code)]
pub mod errors;
#[allow(unused_imports, dead_code)]
pub mod files;
#[allow(unused_imports, dead_code)]
pub mod finishers;
#[allow(unused_imports, dead_code)]
pub mod guild;
#[allow(unused_imports, dead_code)]
pub mod home;
#[allow(unused_imports, dead_code)]
pub mod items;
#[allow(unused_imports, dead_code)]
pub mod itemstats;
#[allow(unused_imports, dead_code)]
pub mod legends;
#[allow(unused_imports, dead_code)]
pub mod mapchests;
#[allow(unused_imports, dead_code)]
pub mod maps;
#[allow(unused_imports, dead_code)]
pub mod masteries;
#[allow(unused_imports, dead_code)]
pub mod materials;
#[allow(unused_imports, dead_code)]
pub mod minis;
#[allow(unused_imports, dead_code)]
pub mod mounts;
#[allow(unused_imports, dead_code)]
pub mod novelties;
#[allow(unused_imports, dead_code)]
pub mod outfits;
#[allow(unused_imports, dead_code)]
pub mod pets;
#[allow(unused_imports, dead_code)]
pub mod professions;
#[allow(unused_imports, dead_code)]
pub mod pvp;
#[allow(unused_imports, dead_code)]
pub mod quaggans;
#[allow(unused_imports, dead_code)]
pub mod quests;
#[allow(unused_imports, dead_code)]
pub mod races;
#[allow(unused_imports, dead_code)]
pub mod raids;
#[allow(unused_imports, dead_code)]
pub mod recipes;
#[allow(unused_imports, dead_code)]
pub mod skills;
#[allow(unused_imports, dead_code)]
pub mod skins;
#[allow(unused_imports, dead_code)]
pub mod specializations;
#[allow(unused_imports, dead_code)]
pub mod stories;
#[allow(unused_imports, dead_code)]
pub mod titles;
#[allow(unused_imports, dead_code)]
pub mod tokeninfo;
#[allow(unused_imports, dead_code)]
pub mod traits;
#[allow(unused_imports, dead_code)]
mod util;
#[allow(unused_imports, dead_code)]
pub mod worldbosses;
#[allow(unused_imports, dead_code)]
pub mod worlds;
#[allow(unused_imports, dead_code)]
pub mod wvw;

// re-export
pub use api::ApiClient;
pub use api::SchemaVersion;
pub use errors::Error;

// type redefinition
type ApiResult<T> = Result<T, Box<dyn std::error::Error>>;

