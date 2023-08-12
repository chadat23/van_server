use server::run_server; // replace `your_crate_name` with the name of your crate in Cargo.toml

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run_server().await
}


// use actix_web::{middleware, web, App, HttpRequest, HttpServer};

// async fn index(req: HttpRequest) -> &'static str {
//     println!("REQ: {req:?}");
//     "Hello world!"
// }

// async fn fan_up(req: HttpRequest) -> &'static str {
//     println!("REQ: {req:?}");
//     "Fan up"
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

//     log::info!("starting HTTP server at http://localhost:8080");

//     HttpServer::new(|| {
//         App::new()
//             // enable logger
//             .wrap(middleware::Logger::default())
//             .service(web::resource("/index.html").to(|| async { "Hello world!" }))
//             .service(web::resource("/fan-up").to(fan_up))
//             .service(web::resource("/").to(index))
//     })
//     .bind(("0.0.0.0", 8080))?
//     .run()
//     .await
// }

// #[cfg(test)]
// mod tests {
//     use actix_web::{body::to_bytes, dev::Service, http, test, web, App, Error};

//     use super::*;

//     #[actix_web::test]
//     async fn test_index() -> Result<(), Error> {
//         let app = App::new().route("/", web::get().to(index));
//         let app = test::init_service(app).await;

//         let req = test::TestRequest::get().uri("/").to_request();
//         let resp = app.call(req).await?;

//         assert_eq!(resp.status(), http::StatusCode::OK);

//         let response_body = resp.into_body();
//         assert_eq!(to_bytes(response_body).await?, r##"Hello world!"##);

//         Ok(())
//     }
// }