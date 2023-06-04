use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    http::{header, HeaderMap, StatusCode},
    response::Html,
    Extension, Form,
};
use serde::Deserialize;

use crate::{
    db::member,
    err::Error,
    form,
    model::{self, state::AppState},
    view, Result,
};
use askama::Template;

fn get_conn(state: &AppState) -> Arc<sqlx::MySqlPool> {
    state.pool.clone()
}

fn redirect(url: &str) -> Result<(StatusCode, HeaderMap, ())> {
    let mut header = HeaderMap::new();
    header.insert(header::LOCATION, url.parse().unwrap());

    Ok((StatusCode::FOUND, header, ()))
}

#[derive(Deserialize)]
pub struct PageQuery {
    pub page: Option<u32>,
    pub msg: Option<String>,
}

pub async fn index(
    Extension(state): Extension<Arc<AppState>>,
    Query(q): Query<PageQuery>,
) -> Result<Html<String>> {
    let conn = get_conn(&state);

    let p = member::list(&conn, q.page.unwrap_or(0)).await?;

    let tpl = view::Home { p, msg: q.msg };
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

pub async fn add_ui() -> Result<Html<String>> {
    let tpl = view::Add {};
    let html = tpl.render().map_err(Error::from)?;

    Ok(Html(html))
}

pub async fn add(
    Extension(state): Extension<Arc<AppState>>,
    Form(frm): Form<form::AddAndEdit>,
) -> Result<(StatusCode, HeaderMap, ())> {
    let conn = get_conn(&state);

    member::add(
        &conn,
        &model::member::Member {
            name: frm.name,
            balance: frm.balance,
            types: frm.types,
            dateline: chrono::Local::now(),
            ..Default::default()
        },
    )
    .await?;

    redirect("/?msg=会员添加成功")
}

pub async fn edit_ui(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<u32>,
) -> Result<Html<String>> {
    let conn = get_conn(&state);

    let m = member::find(&conn, id).await?;

    match m {
        Some(m) => {
            let tpl = view::Edit { m };
            let html = tpl.render().map_err(Error::from)?;
            Ok(Html(html))
        }
        None => Err(Error::not_found("不存在的会员")),
    }
}

pub async fn edit(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<u32>,
    Form(frm): Form<form::AddAndEdit>,
) -> Result<(StatusCode, HeaderMap, ())> {
    let conn = get_conn(&state);

    member::edit(
        &conn,
        &model::member::Member {
            id,
            name: frm.name,
            balance: frm.balance,
            types: frm.types,
            ..Default::default()
        },
    )
    .await?;

    redirect("/?msg=会员修改成功")
}
