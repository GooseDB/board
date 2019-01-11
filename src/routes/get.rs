use crate::routes::{ForumPage, HomePage, ThreadPage};
use crate::DBConn;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn homepage(conn: DBConn) -> Template {
    HomePage::build(&conn.0)
}

#[get("/forum/<id>")]
pub fn forumpage(conn: DBConn, id: usize) -> Template {
    ForumPage::build(&conn.0, id as i32)
}

#[get("/thread/<id>")]
pub fn threadpage(conn: DBConn, id: usize) -> Template {
    ThreadPage::build(&conn.0, id as i32)
}
