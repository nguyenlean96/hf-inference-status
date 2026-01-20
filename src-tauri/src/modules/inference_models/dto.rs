use polars::frame::DataFrame;
use serde::{Deserialize, Serialize};

use crate::models::hf_model_inference::HFModelInferenceStatusRowData;

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
                avatar_url: avatar_urls.get(i).unwrap_or_default().to_string(),
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
