use actix_web::{middleware::Logger, web, App, HttpServer, Responder};
use anyhow::Result;
use dotenv;
use std::env;

mod errors;
mod init;
mod models;
mod routes;
mod validation;

async fn helo() -> impl Responder {
    "yeeeeeeeeee"
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().expect(".env file not found");
    pretty_env_logger::init();

    let address = env::var("ADDRESS").expect("ADDRESS env var unset");
    let (db_pool, redis_pool) = init::init().await?;

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .data(redis_pool.clone())
            .service(
                web::scope("/api").route("/", web::get().to(helo)).service(
                    web::scope("/users/auth")
                        .service(routes::auth::register)
                        .service(routes::auth::login),
                ),
            )
            .wrap(Logger::default())
    })
    .bind(address)?
    .run()
    .await?;

    Ok(())
}
