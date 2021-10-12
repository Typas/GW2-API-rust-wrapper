use crate::SchemaVersion;
use reqwest::RequestBuilder;

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
        SchemaVersion::Time(t) => Some(t.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)),
        SchemaVersion::Latest => Some(String::from("latest")),
    }
}
