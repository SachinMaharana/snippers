use actix_web::http::StatusCode;
use actix_web::{guard, middleware, web, App, Error, HttpResponse, HttpServer};

async fn healthz() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .status(StatusCode::OK)
        .json("{message: everything is fine.}"))
}

async fn home() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .status(StatusCode::OK)
        .body("Hello from Snippers"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix=trace,actix_web=trace,actix_test=trace");
    env_logger::init();

    let endpoint = "127.0.0.1:8080";
    println!("Server listening on: {}", endpoint);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/healthz").route(web::get().to(healthz)))
            .service(web::resource("/").route(web::get().to(home)))
            .default_service(
                web::resource("")
                    .route(web::get().to(|| HttpResponse::NotFound().body("404 Not Found")))
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    })
    .bind(endpoint)?
    .run()
    .await
}
