use serde::Deserialize;

use crate::model::member::MemberTypes;

#[derive(Deserialize)]
pub struct AddAndEdit {
    pub name: String,
    pub balance: u32,
    pub types: MemberTypes,
}

#[derive(Deserialize)]
pub struct Tran {
    pub from_member: String,
    pub to_member: String,
    pub amount: u32,
}
