use async_std::task;

use serde::{Serialize, Deserialize};

use crate::datacli::*;
use crate::model;


#[derive(Serialize, Deserialize)]
pub struct NewNote {
    title: String,
    body: String
}


impl NewNote {
    pub async fn create(self) -> (usize, u64) {
        task::spawn_blocking(move || {
            let conn = establish_connection();

            let (rows, id) = create_note(&conn, &self.title, &self.body);

            println!("inserted rows = {}, id = {}", rows, id);

            (rows, id)
        }).await
    }
}


#[derive(Serialize, Deserialize)]
pub struct NoteRequest {
    pub id: u64
}


impl NoteRequest {
    pub async fn publish(self) -> usize {
        task::spawn_blocking(move || {
            let conn = establish_connection();

            let rows = publish_note(&conn, self.id);

            println!("updated rows = {}", rows);

            rows
        }).await
    }

    pub async fn delete(self) -> usize {
        task::spawn_blocking(move || {
            let conn = establish_connection();

            let rows = delete_note(&conn, self.id);

            println!("deleted rows = {}", rows);

            rows
        }).await
    }

    pub async fn show(self) -> Notes {
        task::spawn_blocking(move || {
            let conn = establish_connection();

            let notes = Notes { notes: get_all_notes(&conn) };

            println!("fetched all notes");

            notes
        }).await
    }

    pub async fn show_published(self) -> Notes {
        task::spawn_blocking(move || {
            let conn = establish_connection();

            let notes = Notes { notes: get_published_notes(&conn) };

            println!("fetched published notes");

            notes
        }).await
    }
}


#[derive(Serialize, Deserialize)]
pub struct Notes {
    notes: Vec<model::Note>
}

