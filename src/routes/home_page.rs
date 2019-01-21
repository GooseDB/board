use crate::routes::{Link, Name};
use crate::tables::Category;
use rocket_contrib::{databases::diesel::MysqlConnection, templates::Template};

#[derive(Serialize)]
pub struct HomePage {
    title: String,
    categories: Vec<(Link, Name)>,
}

impl HomePage {
    pub fn build(conn: &MysqlConnection) -> Template {
        let categories = Category::all(conn)
            .into_iter()
            .map(|category| (format!("/category/{}", category.id()), category.name()))
            .collect();
        Template::render(
            "home_page",
            HomePage {
                title: "Homepage".into(),
                categories,
            },
        )
    }
}
