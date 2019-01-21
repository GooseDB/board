use chrono::NaiveDateTime;
use crate::routes::{P404, P500};
use crate::tables::{Comment, Thread};
use diesel::{self, result::Error, Connection};
use rocket_contrib::{databases::diesel::MysqlConnection, templates::Template};

type Content = String;
type Date = String;

#[derive(Serialize)]
pub struct ThreadPage {
    title: String,
    name: String,
    content: String,
    date: String,
    comments: Vec<(Content, Date)>,
}

impl ThreadPage {
    pub fn build(conn: &MysqlConnection, id: i32) -> Template {
        let result = conn.transaction::<_, Error, _>(|| {
            Ok((Thread::by_id(conn, id).pop(), Comment::by_thread(conn, id)))
        });
        match result {
            Ok((thread, comments)) => {
                let mut comments = comments.into_iter();
                match thread {
                    Some(thread) => Template::render(
                        "thread_page",
                        ThreadPage {
                            title: "Thread".into(),
                            date: format!("{}", thread.date()),
                            name: thread.title(),
                            content: match comments.next() {
                                Some(c) => c.content(),
                                None => "".into(),
                            },
                            comments: comments
                                .map(|comm| (format!("{}", comm.date()), comm.content()))
                                .collect(),
                        },
                    ),
                    None => Template::render("p404", P404 {}),
                }
            }
            Err(_) => Template::render("p500", P500 {}),
        }
    }
}
