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
    pub fn content(self) -> String {
        self.content
    }
    pub fn date(&self) -> NaiveDateTime {
        self.created_at
    }
}

impl Comment {
    pub fn by_thread(conn: &MysqlConnection, thread_id: i32) -> Vec<Comment> {
        use crate::schema::comments;
        comments::dsl::comments
            .filter(comments::thread_id.eq(thread_id))
            .then_order_by(comments::created_at.asc())
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
    pub fn create(conn: &MysqlConnection, new_comment: NewComment, bump_limit: i32) {
        let create_comment = diesel::insert_into(comments::table).values(&new_comment);
        conn.transaction::<_, Error, _>(|| {
            let thread = Thread::by_id(conn, new_comment.thread_id)
                .pop()
                .expect("On ho..");
            match thread.position().cmp(&1) {
                Ordering::Equal => {}
                _ if thread.bump() < bump_limit => {
                    let _ = Thread::lower_until(conn, thread.position())?;
                    diesel::update(threads::table)
                        .filter(threads::id.eq(new_comment.thread_id))
                        .set((threads::position.eq(1), threads::bump.eq(threads::bump + 1)))
                        .execute(conn)?;
                }
                _ => {}
            };
            create_comment.execute(conn)?;
            Ok(())
        }).unwrap();
    }
}
