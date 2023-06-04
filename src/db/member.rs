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

    let mut q = sqlx::QueryBuilder::new("SELECT * FROM member ORDER BY id DESC");
    q.push(" LIMIT ")
        .push_bind(DEFAULT_PAGE_SIZE)
        .push(" OFFSET ")
        .push_bind(page * DEFAULT_PAGE_SIZE);

    let data = q
        .build_query_as()
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
    let mut q = sqlx::QueryBuilder::new("SELECT COUNT(*) FROM member WHERE name=");
    q.push_bind(name);

    if let Some(id) = id {
        q.push(" AND id<>").push_bind(id);
    };

    let count: (i64,) = q
        .build_query_as()
        .fetch_one(conn)
        .await
        .map_err(Error::from)?;

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

pub async fn tran(conn: &sqlx::MySqlPool, t: &model::member::Tran) -> Result<(u64, u64)> {
    let mut tx = conn.begin().await.map_err(Error::from)?;

    let from_aff =
        match sqlx::query("UPDATE member SET balance=balance-? WHERE name=? AND balance>=?")
            .bind(&t.amount)
            .bind(&t.from_member)
            .bind(&t.amount)
            .execute(&mut tx)
            .await
        {
            Ok(r) => r.rows_affected(),
            Err(err) => {
                tx.rollback().await.map_err(Error::from)?;
                return Err(Error::from(err));
            }
        };

    if from_aff < 1 {
        tx.rollback().await.map_err(Error::from)?;
        return Err(Error::tran("转账失败，请检查转出账户是否有足够余额"));
    }

    let to_aff = match sqlx::query("UPDATE member SET balance=balance+? WHERE name=?")
        .bind(&t.amount)
        .bind(&t.to_member)
        .execute(&mut tx)
        .await
    {
        Ok(r) => r.rows_affected(),
        Err(err) => {
            tx.rollback().await.map_err(Error::from)?;
            return Err(Error::from(err));
        }
    };

    tx.commit().await.map_err(Error::from)?;

    Ok((from_aff, to_aff))
}

pub async fn select_in(conn: &sqlx::MySqlPool, ids: &[u32]) -> Result<Vec<model::member::Member>> {
    let mut q = sqlx::QueryBuilder::new("SELECT * FROM member WHERE id IN");
    q.push_tuples(ids.iter(), |mut b, id| {
        b.push_bind(id);
    });

    let ms = q
        .build_query_as()
        .fetch_all(conn)
        .await
        .map_err(Error::from)?;

    Ok(ms)
}
