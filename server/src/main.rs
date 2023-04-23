use actix_cors::Cors;
use actix_web::{get, http, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use hardware::manager::{DeviceManager, Message};
use std::sync::Mutex;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/data")]
async fn data(data: web::Data<ServerState>) -> impl Responder {
    let dev_manager = data.dev_manager.lock().unwrap();
    if let Message::Data(value) = dev_manager.receiver.recv().unwrap() {
        HttpResponse::Ok().body(value.to_string())
    } else {
        HttpResponse::Ok().body("Error retreiving sensor data")
    }
}

struct ServerState {
    dev_manager: Mutex<DeviceManager>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut dev_manager = DeviceManager::new();
    dev_manager.run();
    let state = web::Data::new(ServerState {
        dev_manager: Mutex::new(dev_manager),
    });

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .app_data(state.clone())
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(hello)
            .service(echo)
            .service(data)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
