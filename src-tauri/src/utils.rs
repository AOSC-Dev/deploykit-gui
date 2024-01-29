use eyre::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Recipe {
    variants: Value,
    mirrors: Value,
}

pub async fn get_recpie() -> Result<Recipe> {
    let client = Client::builder().user_agent("deploykit").build()?;
    let recipe = client
        .get("https://releases.aosc.io/manifest/recipe.json")
        .send()
        .await?
        .error_for_status()?
        .json::<Recipe>()
        .await?;

    Ok(recipe)
}
