use actix_web::{dev::Server, middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer};
use anyhow::Result;
use crate::backend::settings::Settings;

use super::route;

pub async fn serve() -> Result<()> {
    let log4rs_path = Settings::log4rs()?;

    log4rs::init_file(
        &log4rs_path, 
        Default::default(),
    )?;

    log::info!("🚀 start server");
    
    run_http_server()?.await?;

    Ok(())
}

fn run_http_server() -> Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())  
            .service(route::get_index)
            .service(route::get_favicon)
            .service(route::get_dist)
            .service(route::get_res)
            .default_service(
                web::route().to(|_:HttpRequest| HttpResponse::NotFound())
            )
    });

    let server = match Settings::local_mode()? {
        true => server.bind(
            ("localhost", Settings::server_port()?),
        ),
        false => server.bind_rustls_0_22(
            ("0.0.0.0", Settings::server_port()?), 
            Settings::read_tls_server_config()?,
        ),
    }?;

    Ok(server.run())
}