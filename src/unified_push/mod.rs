use reqwest::Client;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
struct UnifiedPushResult {
    #[serde(rename = "unifiedpush")]
    unified_push: UnifiedPushInfo,
}

#[derive(Debug, Deserialize)]
struct UnifiedPushInfo {
    version: usize,
}

pub(crate) async fn validate_url(client: &Client, url: &str) -> bool {
    if let Ok(result) = client.get(url).send().await {
        result
            .json::<UnifiedPushResult>()
            .await
            .map(|r| r.unified_push.version == 1)
            .unwrap_or(false)
    } else {
        false
    }
}
