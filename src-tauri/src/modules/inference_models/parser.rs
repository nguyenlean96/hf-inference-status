use scraper::{Html, Selector};

pub fn filter_data_table(html_str: String) -> Option<String> {
    let document = Html::parse_document(&html_str);
    let table_selector =
        Selector::parse("body > div:nth-child(1) > main > div > section > div > div:nth-child(2) > div > div:nth-child(2) > div > table")
            .ok()?;
    let table = document.select(&table_selector).next()?;

    Some(table.html())
}
