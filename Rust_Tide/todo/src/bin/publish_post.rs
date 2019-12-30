use std::env::args;

use diesel::prelude::*;

use todo::*;
use self::datacli::*;
use self::model::Posts;

fn main() {
    use self::schema::posts::dsl::{posts, published};

    let id = args().nth(1).expect("publish_post requires a post id")
        .parse::<u64>().expect("invalid id");

    let conn = establish_connection();

    let post: Posts = posts.find(id).first(&conn).expect("Unable to find post {}");

    diesel::update(posts.find(id))
        .set(published.eq(true))
        .execute(&conn)
        .unwrap();

    println!("Published post: {}", post.title);
}