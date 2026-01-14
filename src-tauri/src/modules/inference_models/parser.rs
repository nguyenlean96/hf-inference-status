use anyhow::Result;
use polars::prelude::*;
use scraper::{Html, Selector};
use std::sync::LazyLock;

use super::prelude::*;

struct TableSelectors {
    row: Selector,
    avatar: Selector,
    long_name: Selector,
    short_name: Selector,
    details_url: Selector,
    instruction_url: Selector,
    provider: Selector,
    input_price: Selector,
    output_price: Selector,
    context_window: Selector,
    latency: Selector,
    throughput: Selector,
    tools: Selector,
    structured: Selector,
}

static SELECTORS: LazyLock<TableSelectors> = LazyLock::new(|| TableSelectors {
    row: Selector::parse("tbody > tr").unwrap(),
    avatar: Selector::parse("td:nth-child(1) > div:nth-child(1) > img").unwrap(),
    long_name: Selector::parse("td:nth-child(1) > span:nth-child(2)").unwrap(),
    short_name: Selector::parse("td:nth-child(1) > span:nth-child(3)").unwrap(),
    details_url: Selector::parse("td:nth-child(1) > div:nth-child(4) > a:nth-child(1)").unwrap(),
    instruction_url: Selector::parse("td:nth-child(1) > div:nth-child(4) > a:nth-child(3)")
        .unwrap(),

    provider: Selector::parse("td:nth-child(2) a").unwrap(),
    input_price: Selector::parse("td:nth-child(3)").unwrap(),
    output_price: Selector::parse("td:nth-child(4)").unwrap(),
    context_window: Selector::parse("td:nth-child(5)").unwrap(),
    latency: Selector::parse("td:nth-child(6)").unwrap(),
    throughput: Selector::parse("td:nth-child(7)").unwrap(),
    tools: Selector::parse("td:nth-child(8)").unwrap(),
    structured: Selector::parse("td:nth-child(9)").unwrap(),
});

pub fn html_table_to_df(table_str: String) -> Result<DataFrame> {
    let table_html = Html::parse_fragment(&table_str);
    let sel = &*SELECTORS;

    // **DO NOT** use: `table_html.select(&sel.row).collect()` here
    // as it collect the actual html and allocate them into heap
    let table_rows = table_html.select(&sel.row);

    let data_size = table_rows.clone().count();

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

    for row in table_rows {
        let Some(model_avatar_url_el) = row.select(&sel.avatar).next() else {
            continue;
        };

        let Some(model_long_name_el) = row.select(&sel.long_name).next() else {
            continue;
        };
        let Some(model_short_name_el) = row.select(&sel.short_name).next() else {
            continue;
        };

        let Some(model_details_url_el) = row.select(&sel.details_url).next() else {
            continue;
        };
        let Some(model_provider_instruction_url_el) = row.select(&sel.instruction_url).next()
        else {
            continue;
        };
        let Some(inference_provider_name_el) = row.select(&sel.provider).next() else {
            continue;
        };
        let Some(input_price_el) = row.select(&sel.input_price).next() else {
            continue;
        };
        let Some(output_price_el) = row.select(&sel.output_price).next() else {
            continue;
        };
        let Some(context_window_el) = row.select(&sel.context_window).next() else {
            continue;
        };
        let Some(latency_el) = row.select(&sel.latency).next() else {
            continue;
        };
        let Some(throughput_el) = row.select(&sel.throughput).next() else {
            continue;
        };
        let Some(tools_el) = row.select(&sel.tools).next() else {
            continue;
        };
        let Some(structured_output_el) = row.select(&sel.structured).next() else {
            continue;
        };

        avatar_urls.push(
            model_avatar_url_el
                .attr("src")
                .unwrap_or_default()
                .to_string(),
        );
        long_names.push(
            model_long_name_el
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string(),
        );
        short_names.push(
            model_short_name_el
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string(),
        );
        model_details_urls.push(
            model_details_url_el
                .attr("href")
                .unwrap_or_default()
                .to_string(),
        );
        model_inference_instruction_urls.push(
            model_provider_instruction_url_el
                .attr("href")
                .unwrap_or_default()
                .to_string(),
        );
        provider_names.push(
            inference_provider_name_el
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string(),
        );
        input_prices_per_1m.push(parse_price(input_price_el).unwrap_or(-1.0));
        output_prices_per_1m.push(parse_price(output_price_el).unwrap_or(-1.0));
        context_window_sizes.push(parse_number::<i64>(context_window_el).unwrap_or(-1));
        latencies.push(parse_number::<f64>(latency_el).unwrap_or(-1.0));
        throughputs_tokens_per_sec.push(parse_number::<i64>(throughput_el).unwrap_or(-1));
        tools_supports.push(matches!(tools_el.text().collect::<String>().trim(), "Yes"));
        structured_output_supports.push(matches!(
            structured_output_el.text().collect::<String>().trim(),
            "Yes"
        ));
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
