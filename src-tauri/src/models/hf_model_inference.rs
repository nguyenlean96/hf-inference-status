use serde::{Deserialize, Serialize};

fn build_raw_id(row_data: &HFModelInferenceStatusRowData) -> String {
    format!(
        "{}:{}:{}",
        row_data
            .model_family
            .as_deref()
            .unwrap_or("unknown")
            .to_lowercase(),
        row_data.short_name.to_lowercase(),
        row_data.provider_name.to_lowercase()
    )
}

fn hash_id(raw_id: &str) -> String {
    blake3::hash(raw_id.as_bytes()).to_hex().to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HFModelInferenceStatus {
    /// Composite ID (Family + Name + Provider)
    pub raw_id: String,

    /// Hash of the composite ID
    pub id: String,

    #[serde(flatten)]
    pub row_data: HFModelInferenceStatusRowData,
}

impl From<HFModelInferenceStatusRowData> for HFModelInferenceStatus {
    fn from(row_data: HFModelInferenceStatusRowData) -> Self {
        let raw_id = build_raw_id(&row_data);

        let id = hash_id(&raw_id);

        Self {
            raw_id,
            id,
            row_data,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HFModelInferenceStatusRowData {
    pub id: String,
    pub avatar_url: Option<String>,
    pub model_family: Option<String>,
    pub short_name: String,
    pub model_details_url: String,
    pub model_inference_instruction_url: String,
    pub provider_name: String,
    pub input_price_per_1m: Option<f64>,
    pub output_price_per_1m: Option<f64>,
    pub context_window_size: Option<i64>,
    pub latency: Option<f64>,
    pub throughput_token_per_sec: Option<i64>,
    pub tools_support: bool,
    pub structured_output_support: bool,
}
