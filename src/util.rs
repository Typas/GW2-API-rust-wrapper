use crate::SchemaVersion;
use reqwest::RequestBuilder;
use chrono::{DateTime, Utc};

pub fn request_common_build(
    req: RequestBuilder,
    key: &Option<String>,
    ver: &SchemaVersion,
) -> RequestBuilder {
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
    ( $fn_name:ident, $struct:ident ) => {
        pub fn $fn_name(self) -> $fn_name::$struct {
            $fn_name::$struct::new(self.client, self.key, self.version)
        }
    };

    ( $fn_name:ident, $struct:ident, Self) => {
        pub fn $fn_name(self) -> $struct {
            $struct::new(self.client, self.key, self.version)
        }
    };

    ( $fn_name:ident, $struct:ident $(, $field:ident : $type:ty )+ ) => {
        pub fn $fn_name(self $(, $field: $type)+ ) -> $struct {
            $struct::new(self.client, self.key, self.version $(,$field)+)
        }
    }
}

macro_rules! new_builder_from_params {
    () => {
        pub(super) fn new(
            client: Client,
            key: Arc<Option<String>>,
            version: Arc<SchemaVersion>
        ) -> Self {
            Self {
                client,
                key,
                version,
            }
        }
    };

    ($($field:tt : $type:ty)+ $(,)? ) => {
        pub(super) fn new(
            client: Client,
            key: Arc<Option<String>>,
            version: Arc<SchemaVersion>
                $(, $field: $type)+)
            -> Self {
            Self {
                client,
                key,
                version,
                $($field,)+
            }
        }
    }
}

macro_rules! trait_from_jsvalue {
    ( $struct:ident ) => {
        impl std::convert::From<serde_json::Value> for $struct {
            fn from(val: serde_json::Value) -> Self {
                serde_json::from_value(val).unwrap()
            }
        }
    };
}

pub(crate) use into_builder;
pub(crate) use new_builder_from_params;
pub(crate) use trait_from_jsvalue;
