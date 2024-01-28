use reqwest::Client;
use eyre::Result;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
struct Recipe {
    variants: Value,
    mirrors: Value,
}

pub async fn get_recpie() -> Result<String> {
    let client = Client::builder().user_agent("deploykit").build()?;
    let resp = client
        .get("https://releases.aosc.io/manifest/recipe.json")
        .send()
        .await?
        .error_for_status()?;

    Ok(resp.text().await?)
}
