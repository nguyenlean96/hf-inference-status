use scraper::{ElementRef, Html, Selector};

pub fn filter_data_table(html_str: String) -> Option<String> {
    let document = Html::parse_document(&html_str);
    let table_selector =
        Selector::parse("body > div:nth-child(1) > main > div > section > div > div:nth-child(2) > div > div:nth-child(2) > div > table")
            .ok()?;
    let table = document.select(&table_selector).next()?;

    Some(table.html())
}

pub fn parse_price(price_el: ElementRef) -> Option<f64> {
    let inner_text = price_el.text().collect::<String>();
    let trimmed = inner_text.trim().replace(" ", "");

    if trimmed == "-" {
        return None;
    }

    let price_str = trimmed.strip_prefix("$").unwrap_or(&trimmed);
    let cleaned_price_str = price_str.replace(",", "");

    cleaned_price_str.parse::<f64>().ok()
}

pub fn parse_number<T: std::str::FromStr>(el: ElementRef) -> Option<T> {
    let inner_text = el.text().collect::<String>();
    let trimmed = inner_text.trim().replace(" ", "");

    if trimmed == "-" {
        return None;
    }
    let cleaned_str = trimmed.replace(",", "");

    cleaned_str.parse::<T>().ok()
}
