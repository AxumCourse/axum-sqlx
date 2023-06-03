use std::sync::Arc;

use axum::response::Html;

use crate::{err::Error, model::state::AppState, view, Result};
use askama::Template;

fn _get_conn(state: &AppState) -> Arc<sqlx::MySqlPool> {
    state.pool.clone()
}

pub async fn index() -> Result<Html<String>> {
    let tpl = view::Home {};
    let html = tpl.render().map_err(Error::from)?;
    Ok(Html(html))
}
