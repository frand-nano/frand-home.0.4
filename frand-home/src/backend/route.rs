use actix_files::NamedFile;
use actix_web::{get, web::Path, HttpRequest, HttpResponse};

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