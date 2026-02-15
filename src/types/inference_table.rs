use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TableColumn {
    Id,
    OriginalOrder,
    AvatarUrl,
    ModelFamily,
    ShortName,
    ModelDetailsUrl,
    ModelInferenceInstructionUrl,
    ProviderName,
    InputPricePer1m,
    OutputPricePer1m,
    ContextWindowSize,
    Latency,
    ThroughputTokenPerSec,
    ToolsSupport,
    StructuredOutputSupport,
}

impl TableColumn {
    pub fn name(&self) -> &'static str {
        match self {
            TableColumn::Id => "ID",
            TableColumn::OriginalOrder => "Original Order",
            TableColumn::AvatarUrl => "Avatar Url",
            TableColumn::ModelFamily => "Model Family",
            TableColumn::ShortName => "Short Name",
            TableColumn::ModelDetailsUrl => "Model Details Url",
            TableColumn::ModelInferenceInstructionUrl => "Model Inference Instruction Url",
            TableColumn::ProviderName => "Provider",
            TableColumn::InputPricePer1m => "Input Price (per 1M tokens)",
            TableColumn::OutputPricePer1m => "Output Price (per 1M tokens)",
            TableColumn::ContextWindowSize => "Context Window (tokens)",
            TableColumn::Latency => "Latency (seconds)",
            TableColumn::ThroughputTokenPerSec => "Throughput (Token/second)",
            TableColumn::ToolsSupport => "Tool calls",
            TableColumn::StructuredOutputSupport => "Structured Output",
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            TableColumn::Id => "id",
            TableColumn::OriginalOrder => "original_order",
            TableColumn::AvatarUrl => "avatar_url",
            TableColumn::ModelFamily => "model_family",
            TableColumn::ShortName => "short_name",
            TableColumn::ModelDetailsUrl => "model_details_url",
            TableColumn::ModelInferenceInstructionUrl => "model_inference_instruction_url",
            TableColumn::ProviderName => "provider_name",
            TableColumn::InputPricePer1m => "input_price_per_1m",
            TableColumn::OutputPricePer1m => "output_price_per_1m",
            TableColumn::ContextWindowSize => "context_window_size",
            TableColumn::Latency => "latency",
            TableColumn::ThroughputTokenPerSec => "throughput_token_per_sec",
            TableColumn::ToolsSupport => "tools_support",
            TableColumn::StructuredOutputSupport => "structured_output_support",
        }
    }
}

impl fmt::Display for TableColumn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SortOrder {
    NotSet,
    Ascending,
    Descending,
}
