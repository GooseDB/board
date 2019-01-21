use rocket::{request::Form, State};

use crate::{
    tables::{NewComment, NewThread},
    DBConn, Settings,
};

#[post("/new_comment", data = "<new_comment>")]
pub fn comment(settings: State<Settings>, conn: DBConn, new_comment: Form<NewComment>) {
    let new_comment = new_comment.into_inner();
    NewComment::create(&conn.0, new_comment, settings.bump_limit());
}

#[post("/thread", data = "<new_thread>")]
pub fn thread(conn: DBConn, new_thread: Form<NewThread>) {
    let new_thread = new_thread.into_inner();
    NewThread::create(&conn.0, new_thread);
}
