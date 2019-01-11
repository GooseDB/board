use rocket::request::Form;

use crate::{
    tables::{NewComment, NewThread},
    DBConn,
};

#[post("/comment", data = "<new_comment>")]
pub fn comment(conn: DBConn, new_comment: Form<NewComment>) {
    let new_comment = new_comment.into_inner();
    NewComment::create(&conn.0, new_comment);
}

#[post("/thread", data = "<new_thread>")]
pub fn thread(conn: DBConn, new_thread: Form<NewThread>) {
    let new_thread = new_thread.into_inner();
    NewThread::create(&conn.0, new_thread);
}
