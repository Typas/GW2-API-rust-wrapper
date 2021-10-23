use super::ApiDataType;
use super::ApiResult;
use std::fmt;

#[allow(dead_code)]
pub struct ApiClient {
    key: Option<String>,
    client: reqwest::blocking::Client,
    version: SchemaVersion,
}

#[allow(dead_code)]
pub struct ApiClientBuilder {
    key: Option<String>,
    client: reqwest::blocking::Client,
    version: SchemaVersion,
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
    pub fn get<T>(&self, endpoint: &EndPoint) -> ApiResult<T>
    where
        T: ApiDataType + serde::de::DeserializeOwned,
    {
        let url = String::from("https://api.guildwars2.com/v2") + &Self::select_url(endpoint);
        let ver = Self::schema_version(&self.version);

        // XXX: I want to combine these to one statement
        let req = self.client.get(&url);
        let req = match &self.key {
            Some(key) => req.bearer_auth(&key),
            None => req,
        };
        let req = match ver {
            Some(ver) => req.header("X-schema-Version", ver),
            None => req,
        };

        let data: T = req.send()?.json()?;

        Ok(data)
    }

    fn select_url(endpoint: &EndPoint) -> String {
        match endpoint {
            _ => todo!(),
        }
    }

    fn schema_version(version: &SchemaVersion) -> Option<String> {
        match version {
            SchemaVersion::Default => None,
            SchemaVersion::Time(_t) => todo!(),
            SchemaVersion::Latest => Some(String::from("latest")),
        }
    }
}

impl ApiClientBuilder {
    /// create a default setting
    pub fn new() -> ApiResult<Self> {
        let client = reqwest::blocking::ClientBuilder::new()
            .https_only(true)
            .build()?;

        let item = ApiClientBuilder {
            key: None,
            client,
            version: SchemaVersion::Latest,
        };

        Ok(item)
    }

    /// change api key of client
    pub fn key(self, key: &str) -> Self {
        let k = key.to_owned();
        ApiClientBuilder {
            key: Some(k),
            client: self.client,
            version: self.version,
        }
    }

    /// change schema version of client
    pub fn schema(self, version: SchemaVersion) -> Self {
        ApiClientBuilder {
            key: self.key,
            client: self.client,
            version,
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

pub enum EndPoint {
    Account,
    Achievements,
    DailyRewards,
    GameMechanics,
    Guild,
    Home,
    Items,
    MapInfo,
    Misc,
    Story,
    PvP,
    TradingPost,
    WvW,
}

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

impl std::error::Error for NotAuthenticatedError {}
