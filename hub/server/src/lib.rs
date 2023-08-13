use std::collections::HashMap;

use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};

#[derive(Debug, Copy, Clone)]
pub enum Device {
    Fan,
    AC,
    Heater,
    ExteriorLight,
    KitchenLight,
    BedroomLight,
    InteriorFan,
}

impl Device {
    pub fn from_instruction(s: &str) -> Option<(&str, Self)> {
        use Device::*;
        let device_set: HashMap<&str, Device> = HashMap::from([
            ("fan", Fan),
            ("ac", AC),
            ("heater", Heater),
            ("exterior light", ExteriorLight),
            ("kitchen light", KitchenLight),
            ("bedroom light", BedroomLight),
            ("interior fan", InteriorFan),
        ]);

        for (key, &value) in device_set.iter() {
            if s.starts_with(key) {
                return Some((key, value))
            }
        }

        None

        // (instruction, None)
        // use Device::*;
        // match s.as_str() {
        //     "exterior fan" => Some(ExteriorFan),
        //     "ac" => Some(AC),
        //     "heater" => Some(Heater),
        //     "exterior light" => Some(ExteriorLight),
        //     "kitchen light" => Some(KitchenLight),
        //     "bedroom light" => Some(BedroomLight),
        //     "interior fan" => Some(InteriorFan),
        //     _ => None,
        // }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Action {
    On,
    Off,
    Up,
    Down,
    Min,
    Max,
    Set,
    NoAction,
}

impl Action {
    pub fn from_instruction(s: &str) -> Option<(&str, Self)> {
        use Action::*;
        let action_set: HashMap<&str, Action> = HashMap::from([
            ("on", On),
            ("off", Off),
            ("up", Up),
            ("down", Down),
            ("min", Min),
            ("max", Max),
            ("set", Set),
        ]);

        for (key, &value) in action_set.iter() {
            if s.starts_with(key) {
                return Some((key, value))
            }
        }

        None
//         use Action::*;
//         match s {
//             "exterior fan" => Some(On),
//             "ac" => Some(Off),
//             "heater" => Some(Up),
//             "exterior light" => Some(Down),
//             "kitchen light" => Some(Min),
//             "bedroom light" => Some(Max),
//             "interior fan" => Some(Set),
//             _ => None,
//         }
    }
}

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {req:?}");
    "Hello world!"
}

async fn instruction(_req: HttpRequest, info: web::Query<HashMap<String, String>>) -> HttpResponse {
    // println!("REQ: {:?}", req);
    let instruction = info.iter().next();

    if instruction.is_none() { return HttpResponse::Ok().body("Oops, we didn't get that.") }

    let instruction = instruction.unwrap().1.clone();

    let device = Device::from_instruction(&instruction);

    let (device_name, device) = match device {
        Some((n, d)) => (n, d),
        None => return HttpResponse::Ok().body("Oops, we didn't get the device."),
    };

    let action_plus = instruction.chars().skip(device_name.len() + 1).collect::<String>();

    let action = Action::from_instruction(&action_plus);

    let (action_name, action) = match action {
        Some((s, a)) => (s, a),
        None => return HttpResponse::Ok().body("Oops, we didn't get the action."),
    };
    let target_plus = action_plus.chars().skip(action_name.len() + 1).collect::<String>();

    let target: Option<i32> = target_plus.parse().ok();

    let target_name = match target {
        Some(n) => n.to_string(),
        None => String::from("None"),
    };

    let result = format!("Device: {}, Action: {}, Target: {}", device_name, action_name, target_name);

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