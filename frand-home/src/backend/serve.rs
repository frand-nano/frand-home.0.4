use anyhow::Result;
use warp::Filter;
use crate::backend::settings::Settings;

pub async fn serve() -> Result<()> {
    let log4rs_path = Settings::log4rs()?;

    log4rs::init_file(
        &log4rs_path, 
        Default::default(),
    )?;

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