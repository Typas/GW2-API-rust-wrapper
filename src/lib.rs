// modules
pub mod account;
pub mod api;
pub mod achievements;
pub mod backstory;
pub mod commerce;
pub mod guild;
pub mod home;
pub mod mounts;
pub mod pvp;
pub mod recipes;
pub mod stories;
pub mod wvw;
pub mod utility;
mod util;


// re-export
pub use account::AccountData;
pub use account::achievements::AchievementsData as AccountAchievementsData;
pub use api::SchemaVersion;
pub use api::ApiClient;

// type redefinition
type ApiResult<T> = Result<T, Box<dyn std::error::Error>>;

// enumerations
pub enum Error {
    NotAuthenticated(api::NotAuthenticatedError),
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
