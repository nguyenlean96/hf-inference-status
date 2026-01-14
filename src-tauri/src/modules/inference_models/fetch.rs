use anyhow::Result;

pub async fn fetch_data() -> Result<String> {
    let response = reqwest::get("https://huggingface.co/inference/models")
        .await?
        .text()
        .await?;

    Ok(response)
}
