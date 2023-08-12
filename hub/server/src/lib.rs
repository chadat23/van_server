use std::collections::HashMap;

use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {req:?}");
    "Hello world!"
}

async fn instruction(req: HttpRequest, info: web::Query<HashMap<String, String>>) -> HttpResponse {
    println!("REQ: {:?}", req);

    let mut result = String::from("Start");

    for (key, value) in info.iter() {
        // result.push_str(value);
        result = result + ", " + key + " = " + value
    }

    HttpResponse::Ok().body(result)
}
pub async fn run_server() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/instruction").to(instruction))
            .service(web::resource("/").to(index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
// //#[actix_web::main]
// pub async fn run_server() -> std::io::Result<()> {
//     env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

//     log::info!("starting HTTP server at http://localhost:8080");

//     HttpServer::new(|| {
//         App::new()
//             // enable logger
//             .wrap(middleware::Logger::default())
//             .service(web::resource("/index.html").to(|| async { "Hello world!" }))
//             .service(web::resource("/instruction").to(instruction))
//             .service(web::resource("/").to(index))
//     })
//     .bind(("0.0.0.0", 8080))?
//     .run()
//     .await
// }

#[cfg(test)]
mod tests {
    use actix_web::{body::to_bytes, dev::Service, http, test, web, App, Error};

    use super::*;

    #[actix_web::test]
    async fn test_index() -> Result<(), Error> {
        let app = App::new().route("/", web::get().to(index));
        let app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await?;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = resp.into_body();
        assert_eq!(to_bytes(response_body).await?, r##"Hello world!"##);

        Ok(())
    }
}