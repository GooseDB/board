use chrono::NaiveDateTime;
use crate::{
    schema::{comments, threads},
    tables::thread::Thread,
};
use diesel::{prelude::*, result::Error};
use rocket_contrib::databases::diesel::MysqlConnection;
use std::cmp::Ordering;

#[derive(Queryable)]
pub struct Comment {
    id: i32,
    thread_id: i32,
    content: String,
    created_at: NaiveDateTime,
}

impl Comment {
    fn by_thread(conn: &MysqlConnection, thread_id: i32) -> Vec<Comment> {
        use crate::schema::comments::dsl::*;
        comments
            .filter(thread_id.eq(thread_id))
            .then_order_by(created_at.desc())
            .load::<Comment>(conn)
            .expect("Oh no..")
    }
}

#[derive(Insertable, FromForm)]
#[table_name = "comments"]
pub struct NewComment {
    thread_id: i32,
    content: String,
}

impl NewComment {
    pub fn create(conn: &MysqlConnection, new_comment: NewComment) {
        let create_comment = diesel::insert_into(comments::table).values(&new_comment);
        conn.transaction::<_, Error, _>(|| {
            let thread = Thread::by_id(conn, new_comment.thread_id)?
                .pop()
                .expect("On ho..");
            match thread.position().cmp(&1) {
                Ordering::Equal => {}
                _ => {
                    let _ = Thread::lower_until(conn, thread.position())?;
                    diesel::update(threads::table)
                        .filter(threads::id.eq(new_comment.thread_id))
                        .set(threads::position.eq(1))
                        .execute(conn)?;
                }
            };
            create_comment.execute(conn)?;
            Ok(())
        }).unwrap();
    }
}
