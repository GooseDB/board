use chrono::NaiveDateTime;
use crate::{
    schema::{forums, threads},
    tables::forum::Forum,
};
use diesel::{prelude::*, result::Error};
use rocket_contrib::databases::diesel::MysqlConnection;
use std::cmp::Ordering;

#[derive(Queryable)]
pub struct Thread {
    id: i32,
    forum_id: i32,
    title: String,
    position: i32,
    created_at: NaiveDateTime,
    bump: i32,
}

impl Thread {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn position(&self) -> i32 {
        self.position
    }
    pub fn bump(&self) -> i32 {
        self.bump
    }
}

impl Thread {
    pub fn by_forum(conn: &MysqlConnection, forum_id: i32) -> Vec<Thread> {
        use crate::schema::threads::dsl::*;
        threads
            .filter(forum_id.eq(forum_id))
            .load::<Thread>(conn)
            .expect("Oh no..")
    }
    pub fn by_id(conn: &MysqlConnection, thread_id: i32) -> QueryResult<Vec<Thread>> {
        use crate::schema::threads::dsl::*;
        threads
            .filter(id.eq(thread_id))
            .limit(1)
            .load::<Thread>(conn)
    }
    pub fn lower_all(conn: &MysqlConnection) -> QueryResult<usize> {
        diesel::update(threads::table)
            .set(threads::position.eq(threads::position + 1))
            .execute(conn)
    }
    pub fn lower_until(conn: &MysqlConnection, position: i32) -> QueryResult<usize> {
        diesel::update(threads::table)
            .set(threads::position.eq(threads::position + 1))
            .filter(threads::position.lt(position))
            .execute(conn)
    }
}

#[derive(Insertable, FromForm)]
#[table_name = "threads"]
pub struct NewThread {
    forum_id: i32,
    title: String,
}

impl NewThread {
    pub fn create(conn: &MysqlConnection, new_thread: NewThread) {
        let create_thread = diesel::insert_into(threads::table).values(&new_thread);
        conn.transaction::<_, Error, _>(|| {
            Thread::lower_all(conn)?;
            create_thread.execute(conn)?;
            let forum = Forum::by_id(conn, new_thread.forum_id)?
                .pop()
                .expect("No such forum");
            match forum.cur_threads_number().cmp(&forum.max_threads_number()) {
                Ordering::Equal => diesel::delete(threads::table)
                    .filter(threads::position.gt(forum.max_threads_number()))
                    .execute(conn),
                _ => diesel::update(forums::table)
                    .set(forums::cur_threads_number.eq(forums::cur_threads_number + 1))
                    .execute(conn),
            }?;
            Ok(())
        }).expect("Oh no..");
    }
}
