use crate::util::request_common_build;

use super::ApiResult;
use reqwest::Client;
use serde_json::Value as JsonValue;
use std::{fmt, sync::Arc};

#[allow(dead_code)]
pub struct ApiClient {
    pub(crate) key: Arc<Option<String>>,
    pub(crate) client: Client,
    pub(crate) version: Arc<SchemaVersion>,
}

#[allow(dead_code)]
pub struct ApiClientBuilder {
    key: Arc<Option<String>>,
    client: Client,
    version: Arc<SchemaVersion>,
}

impl ApiClient {
    /// create a api client without any authentication
    pub fn new() -> ApiResult<Self> {
        Ok(Self::config()?.build())
    }

    /// configure api client with api key and schema version
    pub fn config() -> ApiResult<ApiClientBuilder> {
        ApiClientBuilder::new()
    }

    /// get API date by selecting endpoint
    /// remember to use right type to get the data, or it would panic
    pub async fn free_style(&self, path: &str) -> ApiResult<JsonValue> {
        let url = String::from("https://api.guildwars2.com/v2/") + path;

        // XXX: I want to combine these to one statement
        let req = request_common_build(&self.client, &self.key, &self.version, &url);

        let data = req.send().await?.json().await?;

        Ok(data)
    }

    pub fn account(&self) -> crate::account::Builder {
        self.into()
    }

    pub fn achievements(&self) -> crate::achievements::Builder {
        self.into()
    }

    pub fn backstory(&self) -> crate::backstory::Builder {
        self.into()
    }

    pub fn commerce(&self) -> crate::commerce::Builder {
        self.into()
    }

    pub fn guild(&self) -> crate::guild::Builder {
        self.into()
    }

    pub fn home(&self) -> crate::home::Builder {
        self.into()
    }

    pub fn mounts(&self) -> crate::mounts::Builder {
        self.into()
    }

    pub fn pvp(&self) -> crate::pvp::Builder {
        self.into()
    }

    // pub fn recipes(&self) -> crate::recipes::Builder {
    //     self.into()
    // }

    // pub fn stories(&self) -> crate::stories::Builder {
    //     self.into()
    // }

    // pub fn wvw(&self) -> crate::wvw::Builder {
    //     self.into()
    // }

    // pub fn build(&self) -> crate::build::Builder {
    //     self.into()
    // }

    // pub fn characters(&self) -> crate::characters::Builder {
    //     self.into()
    // }

    // pub fn colors(&self) -> crate::colors::Builder {
    //     self.into()
    // }

    // pub fn continents(&self) -> crate::continents::Builder {
    //     self.into()
    // }

    // pub fn createsubtoken(&self) -> crate::createsubtoken::Builder {
    //     self.into()
    // }

    // pub fn currencies(&self) -> crate::currencies::Builder {
    //     self.into()
    // }

    // pub fn dailycrafting(&self) -> crate::dailycrafting::Builder {
    //     self.into()
    // }

    // pub fn dungeons(&self) -> crate::dungeons::Builder {
    //     self.into()
    // }

    // pub fn emblem(&self) -> crate::emblem::Builder {
    //     self.into()
    // }

    // pub fn files(&self) -> crate::files::Builder {
    //     self.into()
    // }

    // pub fn finishers(&self) -> crate::finishers::Builder {
    //     self.into()
    // }

    // pub fn items(&self) -> crate::items::Builder {
    //     self.into()
    // }

    // pub fn itemstats(&self) -> crate::itemstats::Builder {
    //     self.into()
    // }

    // pub fn length(&self) -> crate::legends::Builder {
    //     self.into()
    // }

    // pub fn mapchests(&self) -> crate::mapchests::Builder {
    //     self.into()
    // }

    // pub fn maps(&self) -> crate::maps::Builder {
    //     self.into()
    // }

    // pub fn legendaryarmory(&self) -> crate::legendaryarmory::Builder {
    //     self.into()
    // }

    // pub fn masteries(&self) -> crate::masteries::Builder {
    //     self.into()
    // }

    // pub fn materials(&self) -> crate::materials::Builder {
    //     self.into()
    // }

    // pub fn minis(&self) -> crate::minis::Builder {
    //     self.into()
    // }

    // pub fn novelties(&self) -> crate::novelties::Builder {
    //     self.into()
    // }

    // pub fn outfits(&self) -> crate::outfits::Builder {
    //     self.into()
    // }

    // pub fn pets(&self) -> crate::pets::Builder {
    //     self.into()
    // }

    // pub fn professions(&self) -> crate::professions::Builder {
    //     self.into()
    // }

    // pub fn quaggans(&self) -> crate::quaggans::Builder {
    //     self.into()
    // }

    // pub fn quests(&self) -> crate::quests::Builder {
    //     self.into()
    // }

    // pub fn races(&self) -> crate::races::Builder {
    //     self.into()
    // }

    // pub fn raids(&self) -> crate::raids::Builder {
    //     self.into()
    // }

    // pub fn skills(&self) -> crate::skills::Builder {
    //     self.into()
    // }

    // pub fn skins(&self) -> crate::skins::Builder {
    //     self.into()
    // }

    // pub fn specializations(&self) -> crate::specializations::Builder {
    //     self.into()
    // }

    // pub fn titles(&self) -> crate::titles::Builder {
    //     self.into()
    // }

    // pub fn tokeninfo(&self) -> crate::tokeninfo::Builder {
    //     self.into()
    // }

    // pub fn traits(&self) -> crate::traits::Builder {
    //     self.into()
    // }

    // pub fn worldbosses(&self) -> crate::worldbosses::Builder {
    //     self.into()
    // }

    // pub fn worlds(&self) -> crate::worlds::Builder {
    //     self.into()
    // }
}

impl ApiClientBuilder {
    /// create a default setting
    pub fn new() -> ApiResult<Self> {
        let client = reqwest::ClientBuilder::new().https_only(true).build()?;

        let item = ApiClientBuilder {
            key: Arc::new(None),
            client,
            version: Arc::new(SchemaVersion::Latest),
        };

        Ok(item)
    }

    /// change api key of client
    pub fn key(self, key: &str) -> Self {
        let k = key.to_owned();
        ApiClientBuilder {
            key: Arc::new(Some(k)),
            client: self.client,
            version: self.version,
        }
    }

    /// change schema version of client
    pub fn schema(self, version: SchemaVersion) -> Self {
        ApiClientBuilder {
            key: self.key,
            client: self.client,
            version: Arc::new(version),
        }
    }

    /// build a api client
    pub fn build(self) -> ApiClient {
        ApiClient {
            key: self.key,
            client: self.client,
            version: self.version,
        }
    }
}

#[derive(Clone)]
pub enum SchemaVersion {
    Default,
    Time(chrono::NaiveDateTime),
    Latest,
}

#[derive(Debug)]
pub struct NotAuthenticatedError;

impl fmt::Display for NotAuthenticatedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "API key is invalid, or void")
    }
}

#[derive(Clone, Copy)]
pub enum Language {
    English,
    Spanish,
    German,
    French,
    Chinese,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            &Self::English => "en",
            &Self::Spanish => "es",
            &Self::German => "de",
            &Self::French => "fr",
            &Self::Chinese => "zh",
        };

        write!(f, "{}", s)
    }
}

impl std::error::Error for NotAuthenticatedError {}
