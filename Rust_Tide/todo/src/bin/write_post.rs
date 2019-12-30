use std::io::{stdin, Read};

use diesel::prelude::*;

use todo::*;
use self::datacli::*;
use self::model::{Posts, NewPost};


pub fn create_post<'a>(conn: &MysqlConnection, title: &'a str, body: &'a str)-> Posts {
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");

    posts::table.order(posts::id.desc()).first(conn).unwrap()
}


fn main() -> std::io::Result<()> {
    let conn = establish_connection();

    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)];    // drop the new character

    println!("\nOk! Let's write {} (Press {} when finished\n", title, EOF);
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(&conn, title, &body);
    println!("\nSaved draft {} with id {}", title, post.id);

    Ok(())
}


#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";