use crate::routes::{Link, Name, P404, P500};
use crate::tables::{Category, Forum};
use diesel::{self, result::Error, Connection};
use rocket_contrib::{databases::diesel::MysqlConnection, templates::Template};

#[derive(Serialize)]
pub struct CategoryPage {
    title: String,
    category: String,
    forums: Vec<(Link, Name)>,
}

impl CategoryPage {
    pub fn build(conn: &MysqlConnection, id: i32) -> Template {
        let result = conn.transaction::<_, Error, _>(|| {
            Ok((
                Category::by_id(conn, id).pop(),
                Forum::by_category(conn, id),
            ))
        });
        match result {
            Ok((category, forums)) => match category {
                Some(category) => Template::render(
                    "category_page",
                    CategoryPage {
                        title: "Category".into(),
                        category: category.name(),
                        forums: forums
                            .into_iter()
                            .map(|forum| (format!("/forum/{}", forum.id()), forum.name()))
                            .collect(),
                    },
                ),
                None => Template::render("p404", P404 {}),
            },
            Err(_) => Template::render("p500", P500 {}),
        }
    }
}
