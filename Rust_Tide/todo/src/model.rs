use diesel::Queryable;
use crate::schema::posts;

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