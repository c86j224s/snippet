use async_std::path::Path;

use http::status::StatusCode;

use tide::{prelude::*, IntoResponse, Request, Response, ResultExt, Server};

use crate::{global::GlobalState, note::NewNote};


pub async fn server_run() -> std::io::Result<()> {
    let database = GlobalState::new("./static_files/").await?;

    let mut app = Server::with_state(database);
    //app.at("/").get(|_| async move { "Hello, world!" });
    app.at("/").get(handle_index);
    app.at("/msg/:id").post(handle_msg_post).get(handle_msg_get);
    app.at("/delay").get(handle_delay);
    app.at("/note").post(handle_note_create);
    app.at("/static/*").get(handle_get_static);
    app.listen("127.0.0.1:9876").await?;
    Ok(())
}



pub async fn handle_msg_get(req: Request<GlobalState>) -> Response {
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

pub async fn handle_msg_post(mut req: Request<GlobalState>) -> Response { 
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

pub async fn handle_delay(req: Request<GlobalState>) -> Response {
    let msgstore = req.state().get_message_storage();

    msgstore.delay().await;
    Response::new(StatusCode::OK.into()).body_string("ok.".to_owned())
}

pub async fn handle_get_static(req: Request<GlobalState>) -> Response {
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

pub async fn handle_index(req: Request<GlobalState>) -> Response {
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

pub async fn handle_note_create(mut req: Request<GlobalState>) -> Response {
     let new_note: NewNote = match req.body_json().await.client_err() {
        Ok(v) => v,
        Err(e) => {
            println!("body json parse failed. e = {:#?}", e);
            return e.into_response()
        }
    };

    new_note.create().await;

    Response::new(StatusCode::OK.into())
}