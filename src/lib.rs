#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod blue;
pub mod models;

pub use blue::Blue;

pub(crate) mod blue_db {
    pub use super::*;
}