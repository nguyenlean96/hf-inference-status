use anyhow::Result;
use polars::prelude::*;
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};

pub fn filter_data_table(html_str: String) -> Option<String> {
    let document = Html::parse_document(&html_str);
    let table_selector =
        Selector::parse("body > div:nth-child(1) > main > div > section > div > div:nth-child(2) > div > div:nth-child(2) > div > table")
            .ok()?;
    let table = document.select(&table_selector).next()?;

    Some(table.html())
}

fn parse_price(price_el: ElementRef) -> Option<f64> {
    let inner_text = price_el.text().collect::<String>();
    let trimmed = inner_text.trim().replace(" ", "");

    if trimmed == "-" {
        return None;
    }

    let price_str = trimmed.strip_prefix("$").unwrap_or(&trimmed);
    let cleaned_price_str = price_str.replace(",", "");

    cleaned_price_str.parse::<f64>().ok()
}

fn parse_number<T: std::str::FromStr>(el: ElementRef) -> Option<T> {
    let inner_text = el.text().collect::<String>();
    let trimmed = inner_text.trim().replace(" ", "");

    if trimmed == "-" {
        return None;
    }
    let cleaned_str = trimmed.replace(",", "");

    cleaned_str.parse::<T>().ok()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceModelStatusRowData {
    pub avatar_url: String,
    pub long_name: String,
    pub short_name: String,
    pub model_details_url: String,
    pub model_inference_instruction_url: String,
    pub provider_name: String,
    pub input_price_per_1m: f64,
    pub output_price_per_1m: f64,
    pub context_window_size: i64,
    pub latency: f64,
    pub throughput_token_per_sec: i64,
    pub tools_support: bool,
    pub structured_output_support: bool,
}

pub fn html_table_to_df(table_str: String) -> Result<DataFrame> {
    let table_html = Html::parse_fragment(&table_str);
    let mut df_headers = Vec::<String>::new();

    let th_selector = Selector::parse("th").unwrap();

    for th in table_html.select(&th_selector) {
        let raw_text = th.text().collect::<Vec<_>>().join(" ").trim().to_string();
        if raw_text.starts_with("Model") || raw_text.starts_with("Provider") {
            df_headers.push(raw_text.split(" ").collect::<Vec<_>>()[0].to_string());
        } else {
            df_headers.push(raw_text);
        }
    }

    let mut data = Vec::<InferenceModelStatusRowData>::new();

    let tbody_row_selector = Selector::parse("tbody > tr").unwrap();
    let model_avatar_selector =
        Selector::parse("td:nth-child(1) > div:nth-child(1) > img").unwrap();
    let model_long_name_selector = Selector::parse("td:nth-child(1) > span:nth-child(2)").unwrap();
    let model_short_name_selector = Selector::parse("td:nth-child(1) > span:nth-child(3)").unwrap();
    let model_details_url_selector =
        Selector::parse("td:nth-child(1) > div:nth-child(4) > a:nth-child(1)").unwrap();
    let model_provider_instruction_url_selector =
        Selector::parse("td:nth-child(1) > div:nth-child(4) > a:nth-child(3)").unwrap();

    let provider_name_selector = Selector::parse("td:nth-child(2) a").unwrap();
    let input_price_selector = Selector::parse("td:nth-child(3)").unwrap();
    let output_price_selector = Selector::parse("td:nth-child(4)").unwrap();
    let context_window_selector = Selector::parse("td:nth-child(5)").unwrap();
    let latency_selector = Selector::parse("td:nth-child(6)").unwrap();
    let throughput_selector = Selector::parse("td:nth-child(7)").unwrap();
    let tools_selector = Selector::parse("td:nth-child(8)").unwrap();
    let structured_selector = Selector::parse("td:nth-child(9)").unwrap();

    for row in table_html.select(&tbody_row_selector) {
        let Some(model_avatar_url_el) = row.select(&model_avatar_selector).next() else {
            continue;
        };

        let Some(model_long_name_el) = row.select(&model_long_name_selector).next() else {
            continue;
        };
        let Some(model_short_name_el) = row.select(&model_short_name_selector).next() else {
            continue;
        };

        let Some(model_details_url_el) = row.select(&model_details_url_selector).next() else {
            continue;
        };
        let Some(model_provider_instruction_url_el) =
            row.select(&model_provider_instruction_url_selector).next()
        else {
            continue;
        };
        let Some(inference_provider_name_el) = row.select(&provider_name_selector).next() else {
            continue;
        };
        let Some(input_price_el) = row.select(&input_price_selector).next() else {
            continue;
        };
        let Some(output_price_el) = row.select(&output_price_selector).next() else {
            continue;
        };
        let Some(context_window_el) = row.select(&context_window_selector).next() else {
            continue;
        };
        let Some(latency_el) = row.select(&latency_selector).next() else {
            continue;
        };
        let Some(throughput_el) = row.select(&throughput_selector).next() else {
            continue;
        };
        let Some(tools_el) = row.select(&tools_selector).next() else {
            continue;
        };
        let Some(structured_output_el) = row.select(&structured_selector).next() else {
            continue;
        };

        data.push(InferenceModelStatusRowData {
            avatar_url: model_avatar_url_el
                .attr("src")
                .unwrap_or_default()
                .to_string(),
            long_name: model_long_name_el
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string(),
            short_name: model_short_name_el
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string(),
            model_details_url: model_details_url_el
                .attr("href")
                .unwrap_or_default()
                .to_string(),
            model_inference_instruction_url: model_provider_instruction_url_el
                .attr("href")
                .unwrap_or_default()
                .to_string(),
            provider_name: inference_provider_name_el
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string(),
            input_price_per_1m: parse_price(input_price_el).unwrap_or(-1.0),
            output_price_per_1m: parse_price(output_price_el).unwrap_or(-1.0),

            context_window_size: parse_number::<i64>(context_window_el).unwrap_or(-1),
            latency: parse_number::<f64>(latency_el).unwrap_or(-1.0),
            throughput_token_per_sec: parse_number::<i64>(throughput_el).unwrap_or(-1),
            tools_support: matches!(tools_el.text().collect::<String>().trim(), "Yes"),
            structured_output_support: matches!(
                structured_output_el.text().collect::<String>().trim(),
                "Yes"
            ),
        });
    }

    let data_size = data.len();
    let mut avatar_urls: Vec<String> = Vec::with_capacity(data_size);
    let mut long_names: Vec<String> = Vec::with_capacity(data_size);
    let mut short_names: Vec<String> = Vec::with_capacity(data_size);
    let mut model_details_urls: Vec<String> = Vec::with_capacity(data_size);
    let mut model_inference_instruction_urls: Vec<String> = Vec::with_capacity(data_size);
    let mut provider_names: Vec<String> = Vec::with_capacity(data_size);
    let mut input_prices_per_1m: Vec<f64> = Vec::with_capacity(data_size);
    let mut output_prices_per_1m: Vec<f64> = Vec::with_capacity(data_size);
    let mut context_window_sizes: Vec<i64> = Vec::with_capacity(data_size);
    let mut latencies: Vec<f64> = Vec::with_capacity(data_size);
    let mut throughputs_tokens_per_sec: Vec<i64> = Vec::with_capacity(data_size);
    let mut tools_supports: Vec<bool> = Vec::with_capacity(data_size);
    let mut structured_output_supports: Vec<bool> = Vec::with_capacity(data_size);

    // Prepare data for DataFrame
    for row in data {
        avatar_urls.push(row.avatar_url);
        long_names.push(row.long_name);
        short_names.push(row.short_name);
        model_details_urls.push(row.model_details_url);
        model_inference_instruction_urls.push(row.model_inference_instruction_url);
        provider_names.push(row.provider_name);
        input_prices_per_1m.push(row.input_price_per_1m);
        output_prices_per_1m.push(row.output_price_per_1m);
        context_window_sizes.push(row.context_window_size);
        latencies.push(row.latency);
        throughputs_tokens_per_sec.push(row.throughput_token_per_sec);
        tools_supports.push(row.tools_support);
        structured_output_supports.push(row.structured_output_support);
    }

    let df = df!(
        "avatar_url" => avatar_urls,
        "long_name" => long_names,
        "short_name" => short_names,
        "model_details_url" => model_details_urls,
        "model_inference_instruction_url" => model_inference_instruction_urls,
        "provider_name" => provider_names,
        "input_price_per_1m" => input_prices_per_1m,
        "output_price_per_1m" => output_prices_per_1m,
        "context_window_size" => context_window_sizes,
        "latency" => latencies,
        "throughput_token_per_sec" => throughputs_tokens_per_sec,
        "tools_support" => tools_supports,
        "structured_output_support" => structured_output_supports,
    )
    .unwrap();

    Ok(df)
}
