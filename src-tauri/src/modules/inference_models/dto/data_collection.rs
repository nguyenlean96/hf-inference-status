use polars::frame::DataFrame;
use serde::{Deserialize, Serialize};

use crate::models::hf_model_inference::HFModelInferenceStatusRowData;
use crate::modules::inference_models::{fetch::ApiResponse, prelude::MLModel};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InferenceModelStatusCollection {
    pub data: Vec<HFModelInferenceStatusRowData>,
}

impl From<&DataFrame> for InferenceModelStatusCollection {
    fn from(df: &DataFrame) -> Self {
        let len = df.height();

        let mut results: Vec<HFModelInferenceStatusRowData> = Vec::with_capacity(len);

        // Get column references once
        let ids = df.column("id").unwrap().str().unwrap();
        let avatar_urls = df.column("avatar_url").unwrap().str().unwrap();
        let model_families = df.column("model_family").unwrap().str().unwrap();
        let short_names = df.column("short_name").unwrap().str().unwrap();
        let model_details_urls = df.column("model_details_url").unwrap().str().unwrap();
        let model_inference_urls = df
            .column("model_inference_instruction_url")
            .unwrap()
            .str()
            .unwrap();
        let provider_names = df.column("provider_name").unwrap().str().unwrap();
        let input_prices = df.column("input_price_per_1m").unwrap().f64().unwrap();
        let output_prices = df.column("output_price_per_1m").unwrap().f64().unwrap();
        let context_sizes = df.column("context_window_size").unwrap().i64().unwrap();
        let latencies = df.column("latency").unwrap().f64().unwrap();
        let throughputs = df
            .column("throughput_token_per_sec")
            .unwrap()
            .i64()
            .unwrap();
        let tools = df.column("tools_support").unwrap().bool().unwrap();
        let structured = df
            .column("structured_output_support")
            .unwrap()
            .bool()
            .unwrap();

        for i in 0..len {
            results.push(HFModelInferenceStatusRowData {
                id: ids.get(i).unwrap_or_default().to_string(),
                avatar_url: Some(avatar_urls.get(i).unwrap_or_default().to_string()),
                model_family: model_families.get(i).and_then(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        Some(s.to_string())
                    }
                }),
                short_name: short_names.get(i).unwrap_or_default().to_string(),
                model_details_url: model_details_urls.get(i).unwrap_or_default().to_string(),
                model_inference_instruction_url: model_inference_urls
                    .get(i)
                    .unwrap_or_default()
                    .to_string(),
                provider_name: provider_names.get(i).unwrap_or_default().to_string(),
                input_price_per_1m: input_prices.get(i),
                output_price_per_1m: output_prices.get(i),
                context_window_size: context_sizes.get(i),
                latency: latencies.get(i),
                throughput_token_per_sec: throughputs.get(i),
                tools_support: tools.get(i).unwrap_or(false),
                structured_output_support: structured.get(i).unwrap_or(false),
            });
        }

        Self { data: results }
    }
}

impl From<ApiResponse<MLModel>> for InferenceModelStatusCollection {
    fn from(api_response: ApiResponse<MLModel>) -> Self {
        let mut data = Vec::new();

        for i in 0..api_response.data.len() {
            let ml_model = &api_response.data[i];
            for provider in ml_model.providers.iter() {
                let model_short_name = ml_model
                    .id
                    .to_string()
                    .replace(format!("{}/", ml_model.owned_by).as_str(), "");
                let model_id =
                    blake3::hash(format!("{}:{}", ml_model.id, provider.provider).as_bytes())
                        .to_hex()
                        .to_string();
                let model_details_url = format!("https://huggingface.co/{}", ml_model.id);
                let model_inference_instruction_url = format!(
                    "{}/?inference_api=true&infference_provider={}",
                    model_details_url.clone(),
                    provider.provider
                );
                data.push(HFModelInferenceStatusRowData {
                    id: model_id,
                    avatar_url: None,
                    model_family: Some(ml_model.owned_by.clone()),
                    short_name: model_short_name,
                    model_details_url: model_details_url.clone(),
                    model_inference_instruction_url,
                    provider_name: provider.provider.clone(),
                    input_price_per_1m: provider.pricing.as_ref().map(|p| p.input),
                    output_price_per_1m: provider.pricing.as_ref().map(|p| p.output),
                    context_window_size: provider.context_length,
                    latency: None,
                    throughput_token_per_sec: None,
                    tools_support: provider.supports_tools,
                    structured_output_support: provider.supports_structured_output,
                });
            }
        }
        Self { data }
    }
}
