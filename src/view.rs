use askama::Template;

use crate::{db::Paginate, model};

#[derive(Template)]
#[template(path = "index.html")]
pub struct Home {
    pub p: Paginate<Vec<model::member::Member>>,
    pub msg: Option<String>,
}

#[derive(Template)]
#[template(path = "detail.html")]
pub struct Detail {
    pub m: model::member::Member,
}

#[derive(Template)]
#[template(path = "add.html")]
pub struct Add {}

#[derive(Template)]
#[template(path = "edit.html")]
pub struct Edit {
    pub m: model::member::Member,
}

#[derive(Template)]
#[template(path = "tran.html")]
pub struct Tran {}

#[derive(Template)]
#[template(path = "select_in.html")]
pub struct SelectIn {
    pub data: Vec<model::member::Member>,
    pub ids: Vec<u32>,
}
