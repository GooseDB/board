use crate::tables::Category;
use rocket_contrib::{databases::diesel::MysqlConnection, templates::Template};

pub struct HomePage {}

impl HomePage {
    pub fn build(conn: &MysqlConnection) -> Template {
        Template::render("no", ())
    }
}
