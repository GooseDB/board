use crate::schema::pictures;
use diesel::prelude::*;
use rocket_contrib::databases::diesel::MysqlConnection;

#[derive(Queryable)]
pub struct Picture {
    id: i32,
    comment_id: i32,
    path_to_picture: String,
}

impl Picture {
    pub fn by_comment(conn: &MysqlConnection, comment_id: i32) -> Vec<Picture> {
        use crate::schema::pictures::dsl::*;
        pictures
            .filter(comment_id.eq(comment_id))
            .load::<Picture>(conn)
            .expect("Oh no..")
    }
}

#[derive(Insertable)]
#[table_name = "pictures"]
pub struct NewPicture {
    comment_id: i32,
    path_to_picture: String,
}

impl NewPicture {
    pub fn create(
        conn: &MysqlConnection,
        path_to_picture: String,
        comment_id: i32,
    ) -> QueryResult<usize> {
        let picture = NewPicture {
            comment_id,
            path_to_picture,
        };
        diesel::insert_into(pictures::table)
            .values(&picture)
            .execute(conn)
    }
}
