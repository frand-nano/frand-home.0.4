use actix_files::NamedFile;
use actix_web::{get, web::{Data, Path, Payload}, HttpRequest, HttpResponse};
use actix_ws::handle;
use tokio::sync::mpsc::UnboundedSender;

use crate::backend::server_socket::{ServerSocketConnection, ServerSocketMessage};

#[get("/")]
pub async fn get_index(
    request: HttpRequest,
) -> HttpResponse {    
    match NamedFile::open_async("./target/dist/index.html").await {
        Ok(response) => response.into_response(&request),
        Err(err) => {
            log::error!("get_index() -> Err({err})");  
            HttpResponse::InternalServerError().finish()  
        },
    }
}

#[get("/frand-home-{path}")]
pub async fn get_dist(
    path: Path<(String,)>,
    request: HttpRequest,
) -> HttpResponse {
    let (path,) = path.into_inner();  

    match NamedFile::open_async(format!("./target/dist/frand-home-{path}")).await {
        Ok(response) => response.into_response(&request),
        Err(err) => {
            log::error!("get_dist(path: {path}) -> Err({err})");  
            HttpResponse::NotFound().finish()   
        },
    }
}

#[get("/favicon.ico")]
pub async fn get_favicon(
    request: HttpRequest,
) -> HttpResponse {
    match NamedFile::open_async("../include/res/favicon.ico").await {
        Ok(response) => response.into_response(&request),
        Err(err) => {
            log::error!("get_favicon() -> Err({err})");  
            HttpResponse::NotFound().finish()    
        },
    }
}

#[get("/res/{path}")]
pub async fn get_res(
    path: Path<(String,)>,
    request: HttpRequest,
) -> HttpResponse {
    let (path,) = path.into_inner();  

    match NamedFile::open_async(format!("../include/res/{path}")).await {
        Ok(response) => response.into_response(&request),
        Err(err) => {
            log::error!("get_res(path: {path}) -> Err({err})");  
            HttpResponse::NotFound().finish()   
        },
    }
}

#[get("/ws")]
pub async fn get_ws(
    request: HttpRequest, 
    stream: Payload,
    new_socket_tx: Data<UnboundedSender<ServerSocketConnection>>,
    socket_tx: Data<UnboundedSender<ServerSocketMessage>>,
) -> actix_web::Result<HttpResponse> {
    let (response, session, stream) = handle(&request, stream)?;

    let socket = ServerSocketConnection::new_start(stream, socket_tx.get_ref().clone(), session);
    
    if let Err(err) = new_socket_tx.send(socket) {
        log::error!("Failed to send ServerSocket: {}", err);
        return Ok(HttpResponse::InternalServerError().body("Internal Server Error"));
    }

    Ok(response)
}