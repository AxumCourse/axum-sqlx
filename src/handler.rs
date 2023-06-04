use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    response::Html,
    Extension,
};
use serde::Deserialize;

use crate::{db::member, err::Error, model::state::AppState, view, Result};
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

    let p = member::list(&conn, q.page.unwrap_or(0)).await?;

    let tpl = view::Home { p };
    let html = tpl.render().map_err(Error::from)?;
    Ok(Html(html))
}

pub async fn detail(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<u32>,
) -> Result<Html<String>> {
    let conn = get_conn(&state);

    let m = member::find(&conn, id).await?;

    match m {
        Some(m) => {
            let tpl = view::Detail { m };
            let html = tpl.render().map_err(Error::from)?;
            Ok(Html(html))
        }
        None => Err(Error::not_found("不存在的会员")),
    }
}
