use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct MLModel {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub owned_by: String,
    pub architecture: Architecture,
    pub providers: Vec<Provider>,
}

#[derive(Clone, Deserialize)]
pub struct Architecture {
    pub input_modalities: Vec<String>,
    pub output_modalities: Vec<String>,
}

#[derive(Clone, Deserialize)]
pub struct Provider {
    pub provider: String,
    pub status: String,
    pub context_length: Option<i64>,
    pub pricing: Option<Pricing>,
    pub supports_tools: bool,
    pub supports_structured_output: bool,
    pub is_model_author: bool,
}

#[derive(Clone, Deserialize)]
pub struct Pricing {
    pub input: f64,
    pub output: f64,
}
