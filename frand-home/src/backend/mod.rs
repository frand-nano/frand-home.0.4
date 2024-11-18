use anyhow::{anyhow, Result};
use settings::Settings;
use warp::Filter;

pub mod settings;

pub async fn serve() -> Result<()> {
    let log4rs_path = Settings::log4rs()?;

    log4rs::init_file(
        &log4rs_path, 
        Default::default(),
    ).map_err(|err| anyhow!("Failed to read log4rs.yml file log4rs_path: {log4rs_path} err: {err}"))?;

    log::info!("🚀 start server");

    let routes = warp::fs::dir("target/dist")
    .with(warp::log("warp"));

    match Settings::local_mode()? {
        true => {
            warp::serve(routes)
            .run(([127, 0, 0, 1], Settings::server_port()?))
            .await;
        },
        false => {
            warp::serve(routes)
            .tls()
            .cert_path(Settings::tls_cert()?)
            .key_path(Settings::tls_key()?)
            .run(([0, 0, 0, 0], Settings::server_port()?))
            .await;
        },
    }
    
    Ok(())
}