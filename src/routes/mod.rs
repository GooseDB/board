pub mod category_page;
pub mod forum_page;
pub mod get;
pub mod home_page;
pub mod post;
pub mod thread_page;

pub(crate) type Link = String;
pub(crate) type Name = String;

#[derive(Serialize)]
pub struct P404 {}
#[derive(Serialize)]
pub struct P500 {}

pub use crate::routes::{
    category_page::CategoryPage, forum_page::ForumPage, home_page::HomePage,
    thread_page::ThreadPage,
};
