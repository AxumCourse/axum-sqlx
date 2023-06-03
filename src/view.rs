use askama::Template;

#[derive(Template)]
#[template(path = "tran.html")]
pub struct Home {}
