use std::env::args;

use diesel::prelude::*;

use todo::*;
use self::datacli::*;

fn main() {
    use self::schema::posts::dsl::*;

    let title_target = args().nth(1).expect("delete_post requires title target");
    let target_pattern = format!("%{}%", title_target);

    let conn = establish_connection();

    let num_deleted = diesel::delete(posts.filter(title.like(target_pattern)))
        .execute(&conn)
        .unwrap();

    println!("deleted {} posts.", num_deleted);
}