#[macro_use]
extern crate diesel;

pub mod handler;

pub mod global;
pub mod inmemory;
pub mod sfs;
pub mod note;

pub mod model;
pub mod schema;
pub mod datacli;