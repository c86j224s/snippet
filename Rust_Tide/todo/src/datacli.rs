use diesel::prelude::*;
use diesel::{MysqlConnection, no_arg_sql_function, types::{Unsigned, Bigint}};
use dotenv::dotenv;
use std::env;

use chrono::{prelude::*, NaiveDateTime};

use crate::model::{Note, NewNote};


no_arg_sql_function!(last_insert_id, Unsigned<Bigint>, "Last Inserted PK");


// pooled???
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn get_all_notes(conn: &MysqlConnection) -> Vec<Note> {
    use crate::schema::note;

    note::table.order(note::id.desc()).get_results(conn).unwrap()
}


pub fn get_published_notes(conn: &MysqlConnection) -> Vec<Note> {
    use crate::schema::note::dsl::*;

    note
        .filter(published.eq(true))
        .order(id.desc())
        .get_results(conn)
        .unwrap()
}


pub fn create_note<'a>(
    conn: &MysqlConnection,
    title: &'a str,
    body: &'a str
) -> (usize, u64) {
    use crate::schema::note;

    let now = diesel::select(diesel::dsl::now).get_result::<NaiveDateTime>(conn).unwrap();

    let new_note = NewNote {
        title: title,
        body: body,
        created: now,
        updated: now
    };

    let rows_inserted = diesel::insert_into(note::table)
        .values(&new_note)
        .execute(conn)
        .unwrap();

    let last_inserted_id: u64 = diesel::select(last_insert_id)
        .first(conn)
        .unwrap();

    (rows_inserted, last_inserted_id)
}


pub fn publish_note(conn: &MysqlConnection, target_id: u64) -> usize {
    use crate::schema::note::dsl::*;

    // todo: change updated 

    let rows_updated = diesel::update(note.filter(id.eq(target_id)))
        .set(published.eq(true))
        .execute(conn)
        .unwrap();

    rows_updated
}


pub fn delete_note(conn: &MysqlConnection, target_id: u64) -> usize {
    use crate::schema::note::dsl::*;

    let rows_deleted = diesel::delete(note.filter(id.eq(target_id)))
        .execute(conn)
        .unwrap();

    rows_deleted
}