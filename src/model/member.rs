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
impl std::fmt::Display for MemberTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            &Self::Normal => String::from("普通会员"),
            &Self::Silver => String::from("白银会员"),
            &Self::Gold => String::from("黄金会员"),
            &Self::Diamond => String::from("钻石会员"),
        };
        write!(f, "{}", s)
    }
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

pub struct Tran {
    pub from_member: String,
    pub to_member: String,
    pub amount: u32,
}
