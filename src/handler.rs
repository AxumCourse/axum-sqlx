use std::sync::Arc;

use axum::{extract::Query, response::Html, Extension};
use serde::Deserialize;

use crate::{db, err::Error, model::state::AppState, view, Result};
use askama::Template;

fn get_conn(state: &AppState) -> Arc<sqlx::MySqlPool> {
    state.pool.clone()
}

#[derive(Deserialize)]
pub struct PageQuery {
    pub page: Option<u32>,
}

pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
    Query(q): Query<PageQuery>,
) -> Result<Html<String>> {
    let conn = get_conn(&state);

    let p = db::list(&conn, q.page.unwrap_or(0)).await?;

    let tpl = view::Home { p };
    let html = tpl.render().map_err(Error::from)?;
    Ok(Html(html))
}
