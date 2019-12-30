use async_std::task;

use serde::{Serialize, Deserialize};

use crate::datacli::*;


#[derive(Serialize, Deserialize)]
pub struct NewNote {
    title: String,
    body: String
}


impl NewNote {
    pub async fn create(self) {
        task::spawn_blocking(|| async move {
            let conn = establish_connection();

            let (rows, id) = create_note(&conn, &self.title, &self.body);

            println!("inserted rows = {}, id = {}", rows, id);

        });
    }
}