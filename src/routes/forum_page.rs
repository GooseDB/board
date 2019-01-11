use crate::tables::Category;
use rocket_contrib::{databases::diesel::MysqlConnection, templates::Template};

pub struct ForumPage {}

impl ForumPage {
    pub fn build(conn: &MysqlConnection, id: i32) -> Template {
        Template::render("no", ())
    }
}
