use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceModelStatusRowData {
    pub avatar_url: String,
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
