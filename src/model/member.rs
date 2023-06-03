use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, sqlx::Type, Clone)]
#[repr(u8)]
pub enum MemberTypes {
    #[default]
    /// 普通会员
    Normal,
    /// 白银会员
    Silver,
    /// 黄金会员
    Gold,
    /// 钻石会员
    Diamond,
}

#[derive(Debug, Default, Deserialize, Serialize, sqlx::FromRow)]
pub struct Member {
    pub id: u32,
    pub name: String,
    pub dateline: chrono::DateTime<chrono::Local>,
    pub balance: u32,
    pub types: MemberTypes,
    pub is_del: bool,
}