use crate::{err::Error, model, Result};

use super::{Paginate, DEFAULT_PAGE_SIZE};

pub async fn list(
    conn: &sqlx::MySqlPool,
    page: u32,
) -> Result<Paginate<Vec<model::member::Member>>> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM member")
        .fetch_one(conn)
        .await
        .map_err(Error::from)?;

    let sql = format!(
        "SELECT * FROM member ORDER BY id DESC LIMIT {} OFFSET {}",
        DEFAULT_PAGE_SIZE,
        page * DEFAULT_PAGE_SIZE
    );
    let data = sqlx::query_as(&sql)
        .fetch_all(conn)
        .await
        .map_err(Error::from)?;

    Ok(Paginate::new(count.0 as u32, page, data))
}

pub async fn find(conn: &sqlx::MySqlPool, id: u32) -> Result<Option<model::member::Member>> {
    let m = sqlx::query_as("SELECT * FROM member WHERE id=?")
        .bind(id)
        .fetch_optional(conn)
        .await
        .map_err(Error::from)?;
    Ok(m)
}
