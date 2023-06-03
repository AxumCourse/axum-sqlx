use serde::Serialize;

use crate::{err::Error, model, Result};

const DEFAULT_PAGE_SIZE: u32 = 30;
#[derive(Serialize)]
pub struct Paginate<T: Serialize> {
    pub total: u32,
    pub total_page: u32,
    pub page: u32,
    pub page_size: u32,
    pub data: T,
}

impl<T: Serialize> Paginate<T> {
    pub fn new(total: u32, page: u32, data: T) -> Self {
        let total_page = f64::ceil(total as f64 / DEFAULT_PAGE_SIZE as f64) as u32;
        Self {
            total,
            page,
            total_page,
            page_size: DEFAULT_PAGE_SIZE,
            data,
        }
    }
    pub fn has_prev(&self) -> bool {
        self.page > 0
    }
    pub fn has_next(&self) -> bool {
        self.page < self.total_page - 1
    }
}

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
