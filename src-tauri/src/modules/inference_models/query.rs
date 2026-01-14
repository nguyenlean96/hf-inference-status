use polars::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortColumn {
    pub column: String,
    pub order: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterQuery {
    pub model: Option<Vec<String>>,
    pub provider: Option<Vec<String>>,
    pub tools: Option<bool>,
    pub structured: Option<bool>,
}

pub fn query_models(
    df: &DataFrame,
    filters: FilterQuery,
    sorts: Vec<SortColumn>,
) -> PolarsResult<DataFrame> {
    let mut lf = df.clone().lazy();

    if let Some(models) = filters.model {
        let expr = models
            .iter()
            .map(|m| col("model").str().contains(lit(m.as_str()), false))
            .reduce(|a, b| a.or(b))
            .unwrap_or(lit(true));
        lf = lf.filter(expr);
    }

    if let Some(tools) = filters.tools {
        lf = lf.filter(col("tools").eq(lit(tools)));
    }

    let sort_exprs: Vec<_> = sorts
        .iter()
        .filter(|s| s.order.is_some())
        .map(|s| col(&s.column))
        .collect();

    let descending: Vec<_> = sorts
        .iter()
        .filter(|s| s.order.is_some())
        .map(|s| s.order.as_ref().map(|o| o == "desc").unwrap_or(false))
        .collect();

    if !sort_exprs.is_empty() {
        lf = lf.sort_by_exprs(
            sort_exprs,
            SortMultipleOptions::default().with_order_descending_multi(descending),
        )
    }

    lf.collect()
}
