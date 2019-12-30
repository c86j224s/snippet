use diesel::prelude::*;

use todo::*;
use self::datacli::*;
use self::model::Posts;

fn main() {
    use self::schema::posts::dsl::*;

    let conn = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Posts>(&conn)
        .expect("Error loading posts");

    println!("displaying {} posts", results.len());

    for post in results {
        println!("{}", post.title);
        println!("-----------");
        println!("{}", post.body);
    }
}