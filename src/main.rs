use actix_web::{middleware::Logger, web, App, HttpServer, Responder};
use anyhow::Result;
use dotenv;
use std::env;

mod db;

async fn helo() -> impl Responder {
    "yeeeeeeeeee"
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().expect(".env file not found");
    pretty_env_logger::init();

    let address = env::var("ADDRESS").expect("ADDRESS env var unset");
    let db_pool = db::init::init().await?;

    HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .route("/", web::get().to(helo))
            .wrap(Logger::default())
    })
    .bind(address)?
    .run()
    .await?;

    Ok(())
}
