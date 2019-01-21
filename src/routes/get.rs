use crate::routes::{CategoryPage, ForumPage, HomePage, ThreadPage};
use crate::DBConn;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn homepage(conn: DBConn) -> Template {
    HomePage::build(&conn.0)
}

#[get("/category/<id>")]
pub fn categorypage(conn: DBConn, id: i32) -> Template {
    CategoryPage::build(&conn.0, id)
}

#[get("/forum/<id>")]
pub fn forumpage(conn: DBConn, id: usize) -> Template {
    ForumPage::build(&conn.0, id as i32)
}

#[get("/thread/<id>")]
pub fn threadpage(conn: DBConn, id: usize) -> Template {
    ThreadPage::build(&conn.0, id as i32)
}
