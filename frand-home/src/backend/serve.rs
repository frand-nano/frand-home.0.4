use actix_web::{dev::Server, middleware::Logger, web::{self, Data}, App, HttpRequest, HttpResponse, HttpServer};
use anyhow::Result;
use frand_web::actix::server_socket::ServerSocketConnection;
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

    let (new_conn_tx, new_conn_rx) = unbounded_channel::<ServerSocketConnection>();
    
    start_socket_server(new_conn_rx);

    run_http_server(new_conn_tx)?.await?;

    Ok(())
}

fn start_socket_server(
    new_conn_rx: UnboundedReceiver<ServerSocketConnection>,
) {
    ActixApp::new(new_conn_rx).start();
}

fn run_http_server(
    new_conn_tx: UnboundedSender<ServerSocketConnection>,
) -> Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
        .app_data(Data::new(new_conn_tx.clone()))
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