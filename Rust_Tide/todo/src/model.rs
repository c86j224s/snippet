use diesel::Queryable;

#[derive(Queryable)]
pub struct Posts {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub published: bool,
}
