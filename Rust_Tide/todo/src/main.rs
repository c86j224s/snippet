use std::collections::HashMap;
use std::future::Future;
use std::sync::{Mutex, Arc};
use std::time::Duration;

use async_std::{fs, path::{Path, PathBuf}, task};

use http::status::StatusCode;

use serde::{Deserialize, Serialize};

use tide::{prelude::*, IntoResponse, Request, Response, ResultExt, Server};


//==============================================================================
struct GlobalState {
    message_storage: MessageStorage,
    static_file_server: StaticFileHandler
}


impl GlobalState {
     async fn new(root_path: &str) -> std::io::Result<Self> {
        Ok(Self {
            message_storage: Default::default(),
            static_file_server: StaticFileHandler::new(root_path).await?
        })
    }

    fn get_message_storage(&self) -> MessageStorage {
        self.message_storage.clone()
    }

    fn get_static_file_server(&self) -> StaticFileHandler {
        self.static_file_server.clone()
    }
}


//==============================================================================
#[derive(Default)]
struct MessageStorage {
    inner: Arc<Mutex<MessageStorageInner>>
}


#[derive(Default)]
struct MessageStorageInner {
    mm: HashMap<usize, Message>,
}


#[derive(Serialize, Deserialize, Clone)]
struct Message {
    author: Option<String>,
    contents: String
}


impl MessageStorage {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }

    async fn set_async(&self, id: usize, msg: Message) {
        let contents = &mut self.inner.lock().unwrap().mm;

        if contents.contains_key(&id) {
            contents.entry(id).and_modify(|v| *v = msg);
            return
        }

        contents.insert(id, msg);
    }

    async fn get_async(&self, id: usize) -> Option<Message> {
        let contents = &self.inner.lock().unwrap().mm;

        match contents.get(&id) {
            Some(v) => Some(v.clone()),
            None => None
        }
    }

    async fn delay(&self) {
        println!("delaying...");
        task::sleep(Duration::from_secs(3)).await;
        println!("done...");
    }

}


//==============================================================================
struct StaticFileHandler {
    inner: Arc<Mutex<StaticFileHandlerInner>>
}

struct StaticFileHandlerInner {
    root_path: PathBuf,
    file_cache: HashMap<PathBuf, Arc<Mutex<String>>>
}

impl StaticFileHandler {
    async fn new(root_path: &str) -> std::io::Result<StaticFileHandler> {
        let abs_root_path = fs::canonicalize(root_path).await?;

        Ok(StaticFileHandler {
            inner: Arc::new(Mutex::new(StaticFileHandlerInner {
                root_path : abs_root_path,
                file_cache : Default::default()
            }))
        })
    }

    fn clone(&self) -> StaticFileHandler {
        StaticFileHandler {
            inner: self.inner.clone()
        }
    }

    fn get_root_path(&self) -> PathBuf {
        self.inner.lock().unwrap().root_path.clone()
    }

    fn contains(&self, k: &Path) -> bool {
        self.inner.lock().unwrap().file_cache.contains_key(k)
    }

    fn update(&mut self, k: PathBuf, v: String) {
        self.inner.lock().unwrap().file_cache.entry(k).and_modify(|e| *e = Arc::new(Mutex::new(v)));
    }
    
    fn insert(&mut self, k: PathBuf, v: String) {
        self.inner.lock().unwrap().file_cache.insert(k, Arc::new(Mutex::new(v)));
    }

    fn get_cached(&self, k: &Path) -> Option<Arc<Mutex<String>>> {
        match self.inner.lock().unwrap().file_cache.get(k) {
            Some(v) => Some(v.clone()),
            None => None
        }
    }

    async fn check_valid_path(&self, input_path: &Path) -> std::io::Result<bool> {
        let cloned = { self.clone() };

        let abs_input_path = fs::canonicalize(input_path).await?;

        Ok(abs_input_path.as_path().starts_with(&cloned.get_root_path()))
    }

    async fn cache_file(&mut self, input_path: &Path) -> std::io::Result<bool> {
        let mut cloned = { self.clone() };

        debug_assert!(cloned.check_valid_path(input_path).await?);

        // 1. load file
        let new_file_data = fs::read_to_string(input_path).await?;

        // 2. build key
        let abs_input_path = fs::canonicalize(input_path).await?;

        // 3. set key value
        if cloned.contains(&abs_input_path) {
            cloned.update(abs_input_path, new_file_data);
        }
        else {
            cloned.insert(abs_input_path, new_file_data);
        }

        Ok(true)
    }

    async fn get_cached_file(&mut self, input_path: &Path) -> std::io::Result<Arc<Mutex<String>>> {
        let mut cloned = { self.clone() };

        if !self.check_valid_path(input_path).await? {
            return Err(std::io::Error::from(std::io::ErrorKind::NotFound))
        }

        let abs_input_path = fs::canonicalize(input_path).await?;

        if !cloned.contains(&abs_input_path) {
            cloned.cache_file(&abs_input_path).await?;
            println!("cached the file : {:?}", abs_input_path);
        }
        
        match cloned.get_cached(&abs_input_path) {
            Some(v) => Ok(v.clone()),
            None => Err(std::io::Error::from(std::io::ErrorKind::NotFound))
        }
    }

    async fn get_static_async(&mut self, input_path: &Path) -> std::io::Result<Arc<Mutex<String>>> {
        self.get_cached_file(input_path).await
    }
}


//==============================================================================
async fn handle_msg_get(req: Request<GlobalState>) -> Response {
    let id = match req.param("id").client_err() {
        Ok(v) => v,
        Err(e) => return e.into_response()
    };

    let msgstore = req.state().get_message_storage();

    if let Some(msg) = msgstore.get_async(id).await {
        match Response::new(StatusCode::OK.into()).body_json(&msg) {
            Ok(v) => v,
            _ => Response::new(StatusCode::INTERNAL_SERVER_ERROR.into())
            
        }
    }
    else {
        Response::new(StatusCode::NOT_FOUND.into())
    }
}

async fn handle_msg_post(mut req: Request<GlobalState>) -> Response { 
    let msg = match req.body_json().await.client_err() {
        Ok(v) => v,
        Err(e) => {
            println!("body json parse failed. e = {:#?}", e);
            return e.into_response()
        }
    };
    let id = match req.param("id").client_err() {
        Ok(v) => v,
        Err(e) => {
            println!("param id is not found. {:#?}", e);
            return e.into_response()
        }
    };

    let msgstore = req.state().get_message_storage();

    msgstore.set_async(id, msg).await;
    Response::new(StatusCode::OK.into()).body_string("".to_owned())
}

async fn handle_delay(req: Request<GlobalState>) -> Response {
    let msgstore = req.state().get_message_storage();

    msgstore.delay().await;
    Response::new(StatusCode::OK.into()).body_string("ok.".to_owned())
}

async fn handle_get_static(req: Request<GlobalState>) -> Response {
    let input_path_buf = req.uri().path().to_owned();
    let refined_path_string = format!("./static_files/{}", &input_path_buf[8..]);
    let rel_path = Path::new(&refined_path_string);

    let mut sfs = req.state().get_static_file_server();

    match sfs.get_static_async(&rel_path).await {
        Ok(v) => {
            Response::new(StatusCode::OK.into()).body_string(v.lock().unwrap().to_owned())
        },
        Err(e) => {
            println!("get static async failed. {:#?}", e);
            Response::new(StatusCode::INTERNAL_SERVER_ERROR.into())
        }
    }
}

async fn handle_index(req: Request<GlobalState>) -> Response {
    let rel_path = Path::new("./static_files/index.html");

    let mut sfs = req.state().get_static_file_server();

    match sfs.get_static_async(&rel_path).await {
        Ok(v) => {
            Response::new(StatusCode::OK.into()).body_string(v.lock().unwrap().to_owned()).set_mime(mime::TEXT_HTML_UTF_8)
        },
        Err(e) => {
            println!("get index async failed. {:#?}", e);
            Response::new(StatusCode::OK.into()).body_string(format!("Hello, Tide! There's no index page."))
        }
    }
}


#[async_std::main]
async fn main() -> std::io::Result<()> {
    let database = GlobalState::new("./static_files/").await?;

    let mut app = Server::with_state(database);
    //app.at("/").get(|_| async move { "Hello, world!" });
    app.at("/").get(handle_index);
    app.at("/msg/:id").post(handle_msg_post).get(handle_msg_get);
    app.at("/delay").get(handle_delay);
    app.at("/static/*").get(handle_get_static);
    app.listen("127.0.0.1:9876").await?;
    Ok(())
}
