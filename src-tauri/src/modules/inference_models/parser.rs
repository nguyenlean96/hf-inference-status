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

    let df = df!(
        "avatar_url" => data.iter().map(|row| row.avatar_url.clone()).collect::<Vec<_>>(),
        "long_name" => data.iter().map(|row| row.long_name.clone()).collect::<Vec<_>>(),
        "short_name" => data.iter().map(|row| row.short_name.clone()).collect::<Vec<_>>(),
        "model_details_url" => data.iter().map(|row| row.model_details_url.clone()).collect::<Vec<_>>(),
        "model_inference_instruction_url" => data.iter().map(|row| row.model_inference_instruction_url.clone()).collect::<Vec<_>>(),
        "provider_name" => data.iter().map(|row| row.provider_name.clone()).collect::<Vec<_>>(),
        "input_price_per_1m" => data.iter().map(|row| row.input_price_per_1m).collect::<Vec<_>>(),
        "output_price_per_1m" => data.iter().map(|row| row.output_price_per_1m).collect::<Vec<_>>(),
        "context_window_size" => data.iter().map(|row| row.context_window_size).collect::<Vec<_>>(),
        "latency" => data.iter().map(|row| row.latency).collect::<Vec<_>>(),
        "throughput_token_per_sec" => data.iter().map(|row| row.throughput_token_per_sec).collect::<Vec<_>>(),
        "tools_support" => data.iter().map(|row| row.tools_support).collect::<Vec<_>>(),
        "structured_output_support" => data.iter().map(|row| row.structured_output_support).collect::<Vec<_>>(),
    ).unwrap();

    Ok(df)
}
