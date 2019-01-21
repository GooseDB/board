use crate::routes::{Link, Name, P404, P500};
use crate::tables::{Forum, Thread};
use diesel::{self, result::Error, Connection};
use rocket_contrib::{databases::diesel::MysqlConnection, templates::Template};

#[derive(Serialize)]
pub struct ForumPage {
    title: String,
    forum: String,
    threads: Vec<(Link, Name)>,
}

impl ForumPage {
    pub fn build(conn: &MysqlConnection, id: i32) -> Template {
        let result = conn.transaction::<_, Error, _>(|| {
            Ok((Forum::by_id(conn, id).pop(), Thread::by_forum(conn, id)))
        });
        match result {
            Ok((forum, threads)) => match forum {
                Some(forum) => Template::render(
                    "forum_page",
                    ForumPage {
                        title: "Forum".into(),
                        forum: forum.name(),
                        threads: threads
                            .into_iter()
                            .map(|thread| (format!("/thread/{}", thread.id()), thread.title()))
                            .collect(),
                    },
                ),
                None => Template::render("p404", P404{}),
            },
            Err(_) => Template::render("p500", P500{}),
        }
    }
}
