mod common;

#[cfg(not(target_arch = "wasm32"))]
mod backend;

#[cfg(target_arch = "wasm32")]
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    common::render::render();
}

#[cfg(not(target_arch = "wasm32"))]
#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    backend::serve::serve().await
}