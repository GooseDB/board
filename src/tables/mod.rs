mod category;
mod comment;
mod forum;
mod picture;
mod thread;

pub use crate::tables::{
    category::Category,
    comment::{Comment, NewComment},
    forum::Forum,
    picture::{NewPicture, Picture},
    thread::{NewThread, Thread},
};
