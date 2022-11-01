#[macro_use]
extern crate rocket;

mod database;
mod app;
mod services;
mod grpc;
mod http;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let _ = app::App::new()
        .init()
        .await
        .expect("failed to init app")
        .run()
        .await;

    Ok(())
}
