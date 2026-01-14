pub mod parser;
pub mod prelude;
pub mod query;

use anyhow::Result;

pub async fn get_inference_model_status() -> Result<String> {
    let response = reqwest::get("https://huggingface.co/inference/models")
        .await?
        .text()
        .await?;

    Ok(response)
}
