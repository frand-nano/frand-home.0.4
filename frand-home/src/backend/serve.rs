use actix_web::{dev::Server, middleware::Logger, web::{self, Data}, App, HttpRequest, HttpResponse, HttpServer};
use anyhow::Result;
use frand_web::actix::server_socket::{ServerSocket, ServerSocketConnection};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use crate::backend::{settings::Settings, actix_app::ActixApp};

use super::route;

pub async fn serve() -> Result<()> {
    let log4rs_path = Settings::log4rs()?;

    log4rs::init_file(
        &log4rs_path, 
        Default::default(),
    )?;

    log::info!("🚀 start server");

    let (new_socket_tx, new_socket_rx) = unbounded_channel::<ServerSocketConnection>();
    
    run_socket_server(new_socket_rx);
    run_http_server(new_socket_tx)?.await?;

    Ok(())
}

fn run_socket_server(
    new_socket_rx: UnboundedReceiver<ServerSocketConnection>,
) {
    let server_socket = ServerSocket::new(new_socket_rx);
    let socket_server = ActixApp::new(server_socket);
    socket_server.run();
}

fn run_http_server(
    new_socket_tx: UnboundedSender<ServerSocketConnection>,
) -> Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
        .app_data(Data::new(new_socket_tx.clone()))
        .wrap(Logger::default())  
        .service(route::get_index)
        .service(route::get_favicon)
        .service(route::get_dist)
        .service(route::get_res)
        .service(route::get_ws)
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