pub mod parser;

use anyhow::Result;

pub use parser::filter_data_table;

pub async fn get_inference_model_status() -> Result<String> {
    let response = reqwest::get("https://huggingface.co/inference/models")
        .await?
        .text()
        .await?;

    Ok(response)
}
