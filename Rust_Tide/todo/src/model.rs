use chrono::{prelude::*, NaiveDateTime};

use diesel::Queryable;

use serde::{Serialize, Deserialize};

use crate::schema::{posts, note};

#[derive(Queryable)]
pub struct Posts {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub published: bool,
}


#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}


#[derive(Serialize, Deserialize)]
#[derive(Queryable)]
pub struct Note {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}


#[derive(Insertable)]
#[table_name="note"]
pub struct NewNote<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime, 
}