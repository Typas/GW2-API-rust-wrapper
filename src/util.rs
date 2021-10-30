use crate::SchemaVersion;
use chrono::{DateTime, Utc};

pub(crate) fn request_common_build(
    client: &reqwest::Client,
    key: &Option<String>,
    ver: &SchemaVersion,
    url: &str,
) -> reqwest::RequestBuilder {
    let req = client.get(url);
    let req = match key {
        Some(key) => req.bearer_auth(&key),
        None => req,
    };
    let ver = schema_version(ver);

    match ver {
        Some(ver) => req.header("X-schema-Version", ver),
        None => req,
    }
}

fn schema_version(version: &SchemaVersion) -> Option<String> {
    match version {
        SchemaVersion::Default => None,
        SchemaVersion::Time(t) => Some(DateTime::<Utc>::from_utc(t.clone(), Utc).to_rfc3339_opts(chrono::SecondsFormat::Secs, true)),
        SchemaVersion::Latest => Some(String::from("latest")),
    }
}

macro_rules! into_builder {
    ( $fn_name:ident, $struct:path ) => {
        pub fn $fn_name(self) -> $struct {
            self.into()
        }
    };
}

macro_rules! trait_try_from_jsonvalue {
    ( $struct:ident ) => {
        impl TryFrom<serde_json::Value> for $struct {
            type Error = serde_json::Error;

            fn try_from(val: serde_json::Value) -> Result<Self, Self::Error> {
                serde_json::from_value(val)
            }
        }


    };
}

pub(crate) use into_builder;
pub(crate) use trait_try_from_jsonvalue;
