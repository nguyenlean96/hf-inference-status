use anyhow::Result;
use serde::Deserialize;
use serde::de::DeserializeOwned;

static SITE_URL: &str = "https://huggingface.co/inference/models";
static API_URL: &str = "https://huggingface.co/api/models";

#[derive(Deserialize)]
pub struct ApiResponse<T> {
    pub data: Vec<T>,
}

pub async fn api_fetcher<T>() -> Result<Vec<T>>
where
    T: DeserializeOwned,
{
    let response: ApiResponse<T> = reqwest::get(API_URL).await?.json().await?;

    Ok(response.data)
}

pub async fn site_fetcher() -> Result<String> {
    let response = reqwest::get(SITE_URL).await?.text().await?;

    Ok(response)
}
