use diesel::prelude::*;
use rocket_contrib::databases::diesel::MysqlConnection;

#[derive(Queryable)]
pub struct Category {
    id: i32,
    name: String,
}

impl Category {
    pub fn all(conn: &MysqlConnection) -> Vec<Category> {
        use crate::schema::categories::dsl::*;
        categories.load::<Category>(conn).expect("Oh no..")
    }
    pub fn by_id(conn: &MysqlConnection, id: i32) -> Vec<Category> {
        use crate::schema::categories;
        categories::dsl::categories
            .filter(categories::id.eq(id))
            .load::<Category>(conn)
            .expect("Oh no..")
    }
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn name(self) -> String {
        self.name
    }
}
