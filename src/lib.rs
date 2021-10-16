// modules
#[allow(dead_code, unused_imports)]
pub mod account;
#[allow(dead_code, unused_imports)]
pub mod achievements;
#[allow(dead_code, unused_imports)]
pub mod api;
#[allow(dead_code, unused_imports)]
pub mod backstory;
#[allow(dead_code, unused_imports)]
pub mod commerce;
#[allow(dead_code, unused_imports)]
pub mod guild;
#[allow(dead_code, unused_imports)]
pub mod home;
#[allow(dead_code, unused_imports)]
pub mod mounts;
#[allow(dead_code, unused_imports)]
pub mod pvp;
#[allow(dead_code, unused_imports)]
pub mod recipes;
#[allow(dead_code, unused_imports)]
pub mod stories;
#[allow(dead_code, unused_imports)]
mod util;
#[allow(dead_code, unused_imports)]
pub mod utility;
#[allow(dead_code, unused_imports)]
pub mod wvw;

// re-export
pub use api::ApiClient;
pub use api::SchemaVersion;

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
