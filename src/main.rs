#[macro_use]
extern crate diesel;

use actix_web::http::StatusCode;
use actix_web::{error, middleware, web, App, Error, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

async fn healthz() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().status(StatusCode::OK).json("ok"))
}

async fn home() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .status(StatusCode::OK)
        .body("Hello from Snippers"))
}

#[derive(Serialize, Deserialize)]
struct ShowSnippet {
    id: String,
}

async fn show_snippet(query: web::Query<ShowSnippet>) -> impl Responder {
    let snippet_id = &query.id;
    match snippet_id.parse::<u32>().map(|id| id > 1) {
        Ok(snippet_id) => {
            if snippet_id {
                HttpResponse::Ok()
                    .status(StatusCode::OK)
                    .body("Display a Snippet")
            } else {
                HttpResponse::from_error(error::ErrorBadRequest("ID is less than 1"))
            }
        }
        Err(e) => {
            HttpResponse::from_error(error::ErrorBadRequest(format!("Unable to Parse ID: {}", e)))
        }
    }
}

async fn create_snippet() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .status(StatusCode::OK)
        .body("Create a New Snippet"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var(
        "RUST_LOG",
        "actix=info,actix_web=info,actix_test=info,diesel=debug",
    );
    env_logger::init();
    dotenv().ok();

    let connect_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    // let manager = ConnectionManager::<Sql>

    let endpoint = "127.0.0.1:8080";
    println!("Server listening on: {}", endpoint);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/healthz").route(web::get().to(healthz)))
            .service(web::resource("/").route(web::get().to(home)))
            .service(web::resource("/snippet").route(web::get().to(show_snippet)))
            .service(
                web::resource("/snippet/create")
                    .route(web::post().to(create_snippet))
                    .default_service(web::to(|| async {
                        HttpResponse::MethodNotAllowed()
                            .set_header("allow", "post")
                            .finish()
                    })),
            )
            .default_service(web::route().to(web::HttpResponse::NotFound))
    })
    .bind(endpoint)?
    .run()
    .await
}
