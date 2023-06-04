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

pub async fn exists(conn: &sqlx::MySqlPool, name: &str, id: Option<u32>) -> Result<bool> {
    let sql = "SELECT COUNT(*) FROM member WHERE name=?";
    let with_id_sql: String;

    let q = match id {
        Some(id) => {
            with_id_sql = format!("{} AND id<>?", sql);
            sqlx::query_as(&with_id_sql).bind(&name).bind(id)
        }
        None => sqlx::query_as(sql).bind(&name),
    };

    let count: (i64,) = q.fetch_one(conn).await.map_err(Error::from)?;

    Ok(count.0 > 0)
}

pub async fn add(conn: &sqlx::MySqlPool, m: &model::member::Member) -> Result<u32> {
    if exists(conn, &m.name, None).await? {
        return Err(Error::exists("同名的会员已存在"));
    }

    let id = sqlx::query(
        "INSERT INTO `member` (name, dateline, balance, types,is_del) VALUES(?,?,?,?,?)",
    )
    .bind(&m.name)
    .bind(&m.dateline)
    .bind(&m.balance)
    .bind(&m.types)
    .bind(&m.is_del)
    .execute(conn)
    .await
    .map_err(Error::from)?
    .last_insert_id();

    Ok(id as u32)
}

pub async fn edit(conn: &sqlx::MySqlPool, m: &model::member::Member) -> Result<u64> {
    if exists(conn, &m.name, Some(m.id)).await? {
        return Err(Error::exists("同名的会员已存在"));
    }

    let aff = sqlx::query("UPDATE `member` SET name=?,balance=?,types=? WHERE id=?")
        .bind(&m.name)
        .bind(&m.balance)
        .bind(&m.types)
        .bind(&m.id)
        .execute(conn)
        .await
        .map_err(Error::from)?
        .rows_affected();

    Ok(aff)
}

pub async fn del(conn: &sqlx::MySqlPool, id: u32) -> Result<u64> {
    let aff = sqlx::query("UPDATE member SET is_del=true WHERE id=?")
        .bind(id)
        .execute(conn)
        .await
        .map_err(Error::from)?
        .rows_affected();
    Ok(aff)
}

pub async fn real_del(conn: &sqlx::MySqlPool, id: u32) -> Result<u64> {
    let aff = sqlx::query("DELETE FROM member WHERE id=?")
        .bind(id)
        .execute(conn)
        .await
        .map_err(Error::from)?
        .rows_affected();
    Ok(aff)
}
