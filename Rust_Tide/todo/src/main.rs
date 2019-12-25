use std::collections::HashMap;
use std::future::Future;
use std::sync::Mutex;

use http::status::StatusCode;

use serde::{Deserialize, Serialize};

use tide::{prelude::*, IntoResponse, Request, Response, Result, ResultExt, Server};


#[derive(Default)]
struct Database {
    contents: Mutex<HashMap<usize, Message>>
}


#[derive(Serialize, Deserialize, Clone)]
struct Message {
    author: Option<String>,
    contents: String
}

impl Database {
    fn set(&self, id: usize, msg: Message) {
        let mut contents = self.contents.lock().unwrap();

        if contents.contains_key(&id) {
            contents.entry(id).and_modify(|v| *v = msg);
            return
        }

        contents.insert(id, msg);
    }

    fn get(&self, id: usize) -> Option<Message> {
        let mut contents = self.contents.lock().unwrap();

        match contents.get(&id) {
            Some(v) => Some(v.clone()),
            None => None
        }
    }
}

async fn handle_post(mut req: Request<Database>) -> Response { 
    let mut msg = match req.body_json().await.client_err() {
        Ok(v) => v,
        Err(e) => return e.into_response()
    };
    let id = match req.param("id").client_err() {
        Ok(v) => v,
        Err(e) =>  return e.into_response()
    };

    req.state().set(id, msg);
    Response::new(StatusCode::OK.into()).body_string("".to_owned())
}

async fn handle_get(mut req: Request<Database>) -> Response {
    let id = match req.param("id").client_err() {
        Ok(v) => v,
        Err(e) => return e.into_response()
    };

    if let Some(msg) = req.state().get(id) {
        match Response::new(StatusCode::OK.into()).body_json(&msg) {
            Ok(v) => v,
            _ => Response::new(StatusCode::INTERNAL_SERVER_ERROR.into())
            
        }
    }
    else {
        Response::new(StatusCode::NOT_FOUND.into())
    }
}


#[async_std::main]
async fn main() -> std::io::Result<()> {
    let mut app = Server::with_state(Database::default());
    app.at("/").get(|_| async move { "Hello, world!" });
    app.at("/message/:id").post(handle_post).get(handle_get);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
