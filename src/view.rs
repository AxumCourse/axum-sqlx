use askama::Template;

use crate::{db::Paginate, model};

#[derive(Template)]
#[template(path = "index.html")]
pub struct Home {
    pub p: Paginate<Vec<model::member::Member>>,
}
