#[macro_use]
extern crate diesel;

use actix_web::http::StatusCode;
use actix_web::{error, middleware, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

async fn healthz() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().status(StatusCode::OK).json("ok"))
}

async fn home() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .status(StatusCode::OK)
        .body("Hello from Snippers"))
}

mod actions;
mod models;
mod schema;

#[derive(Serialize, Deserialize)]
struct SnippetId {
    id: String,
}

async fn get_snippet(
    query: web::Query<SnippetId>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let snippet_id = &query.id;
    let valid_id = snippet_id.parse::<i32>().map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::from_error(error::ErrorBadRequest(format!("Unable to Parse ID: {}", e)))
    })?;
    if valid_id < 1 {
        Ok(HttpResponse::from_error(error::ErrorBadRequest(
            "ID is less than 1",
        )))
    } else {
        let conn = pool
            .get()
            .expect("Couldn't get DB connection from MySql Pool");

        let snippet = web::block(move || actions::find_snippet_by_id(valid_id, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;
        if let Some(snippet) = snippet {
            Ok(HttpResponse::Ok().json(snippet))
        } else {
            Ok(HttpResponse::NotFound().body(format!("No snippet found with id: {}", snippet_id)))
        }
    }
}

async fn create_snippet(
    pool: web::Data<DbPool>,
    snippet_data: web::Json<models::NewSnippet>,
) -> Result<HttpResponse, Error> {
    let conn = pool
        .get()
        .expect("Couldn't get DB connection from MySql Pool");

    let snippet = web::block(move || {
        actions::insert_new_snippet(
            snippet_data.title.as_str(),
            snippet_data.content.as_str(),
            &conn,
        )
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().status(StatusCode::OK).json(snippet))
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
    let manager = ConnectionManager::<MysqlConnection>::new(connect_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let endpoint = "127.0.0.1:8080";
    println!("Server listening on: {}", endpoint);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/healthz").route(web::get().to(healthz)))
            .service(web::resource("/").route(web::get().to(home)))
            .service(web::resource("/snippet").route(web::get().to(get_snippet)))
            // .service(create_snippet)
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
