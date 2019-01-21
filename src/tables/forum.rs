use diesel::prelude::*;
use rocket_contrib::databases::diesel::MysqlConnection;

#[derive(Queryable)]
pub struct Forum {
    id: i32,
    name: String,
    category_id: i32,
    max_threads_number: i32,
    cur_threads_number: i32,
}

impl Forum {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn name(self) -> String {
        self.name
    }
    pub fn max_threads_number(&self) -> i32 {
        self.max_threads_number
    }
    pub fn cur_threads_number(&self) -> i32 {
        self.max_threads_number
    }
}

impl Forum {
    pub fn by_category(conn: &MysqlConnection, category_id: i32) -> Vec<Forum> {
        use crate::schema::forums;
        forums::dsl::forums
            .filter(forums::category_id.eq(category_id))
            .load::<Forum>(conn)
            .expect("Oh no..")
    }
    pub fn by_id(conn: &MysqlConnection, id: i32) -> Vec<Forum> {
        use crate::schema::forums;
        forums::dsl::forums
            .filter(forums::id.eq(id))
            .limit(1)
            .load::<Forum>(conn)
            .expect("Oh no..")
    }
}
