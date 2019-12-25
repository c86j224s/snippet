use std::collections::HashMap;
use std::future::Future;
use std::sync::{Mutex, Arc};

use async_std::{fs, path::{Path, PathBuf}};

use http::status::StatusCode;

use serde::{Deserialize, Serialize};

use tide::{prelude::*, IntoResponse, Request, Response, ResultExt, Server};


struct Database {
    contents: Mutex<HashMap<usize, Message>>,
    static_file_server: Mutex<StaticFileHandler>
}


#[derive(Serialize, Deserialize, Clone)]
struct Message {
    author: Option<String>,
    contents: String
}

impl Database {
    async fn new(root_path: &str) -> std::io::Result<Database> {
        Ok(Database {
            contents: Default::default(),
            static_file_server: Mutex::new(StaticFileHandler::new(root_path).await?)
        })
    }

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


struct StaticFileHandler {
    root_path: PathBuf,
    file_cache: HashMap<PathBuf, Arc<Mutex<String>>>
}

impl StaticFileHandler {
    async fn new(root_path: &str) -> std::io::Result<StaticFileHandler> {
        let abs_root_path = fs::canonicalize(root_path).await?;

        Ok(StaticFileHandler {
            root_path : abs_root_path,
            file_cache : Default::default()
        })
    }

    async fn check_valid_path(&self, input_path: &Path) -> std::io::Result<bool> {
        let abs_input_path = fs::canonicalize(input_path).await?;

        Ok(abs_input_path.as_path().starts_with(&self.root_path))
    }

    async fn cache_file(&mut self, input_path: &Path) -> std::io::Result<bool> {
        debug_assert!(self.check_valid_path(input_path).await?);

        // 1. load file
        let new_file_data = Arc::new(Mutex::new(fs::read_to_string(input_path).await?));

        // 2. build key
        let abs_input_path = fs::canonicalize(input_path).await?;

        // 3. set key value
        if self.file_cache.contains_key(&abs_input_path) {
            self.file_cache.entry(abs_input_path).and_modify(|e| *e = new_file_data);
        }
        else {
            self.file_cache.insert(abs_input_path, new_file_data);
        }

        Ok(true)
    }

    async fn get_cached_file(&self, input_path: &Path) -> std::io::Result<Arc<Mutex<String>>> {
        if !self.check_valid_path(input_path).await? {
            return Err(std::io::Error::from(std::io::ErrorKind::NotFound))
        }

        let abs_input_path = fs::canonicalize(input_path).await?;

        if !self.file_cache.contains_key(&abs_input_path) {
            self.cache_file(&abs_input_path).await?;
        }
        
        match self.file_cache.get(&abs_input_path) {
            Some(v) => Ok(v.clone()),
            None => Err(std::io::Error::from(std::io::ErrorKind::NotFound))
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

async fn handle_get_static(mut req: Request<Database>) -> Response {
    let rel_path_buf = req.uri().path().to_owned();
    let rel_path = Path::new(&rel_path_buf[7..]);

    let mut sfs = req.state().static_file_server.lock().unwrap();

    match sfs.get_cached_file(&rel_path).await {
        //Ok(v) => Response::new(StatusCode::OK.into()).body_string(*v),
        Ok(v) => Response::new(StatusCode::OK.into()),
        _ => Response::new(StatusCode::INTERNAL_SERVER_ERROR.into())
    }
}



#[async_std::main]
async fn main() -> std::io::Result<()> {
    let database = Database::new("./static_files/").await?;

    let mut app = Server::with_state(database);
    app.at("/").get(|_| async move { "Hello, world!" });
    app.at("/message/:id").post(handle_post).get(handle_get);
    app.at("/static/*").get(handle_get_static);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
