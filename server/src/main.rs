use actix_cors::Cors;
use actix_web::{get, http, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use hardware::manager::{DeviceManager, Message};
use std::sync::Mutex;
use systemstat::{Platform, System};

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

#[get("/devices")]
async fn devices() -> impl Responder {
    HttpResponse::InternalServerError().body("Not implemented")
}

#[get("/stats/{stat}")]
async fn stats(stat: web::Path<String>) -> impl Responder {
    let sys = System::new();
    match stat.as_str() {
        "cpu_load" => {
            match sys.cpu_load_aggregate() {
                Ok(cpu) => {
                    let cpu = cpu.done().unwrap();
                    HttpResponse::Ok().json(cpu)
                }
                Err(e) => HttpResponse::BadRequest().body(format!("{}", e)),
            }
        },
        "memory" => {
            match sys.memory() {
                Ok(mem) => {
                    HttpResponse::Ok().json(mem)
                },
                Err(e) => HttpResponse::BadRequest().body(format!("{}", e)),
            }
        },
        "uptime" => {
            match sys.uptime() {
                Ok(uptime) => {
                    HttpResponse::Ok().json(uptime)
                },
                Err(e) => HttpResponse::BadRequest().body(format!("{}", e)),
            }
        }
        "socket" => {
            match sys.socket_stats() {
                Ok(stats) => {
                    HttpResponse::Ok().json(stats)
                },
                Err(e) => HttpResponse::BadRequest().body(format!("{}", e)),
            }
        }
        _ => {
            HttpResponse::BadRequest().body("Error")
        },
    }
}

enum _ServerError {}

struct ServerState {
    dev_manager: Mutex<DeviceManager>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(ServerState {
        dev_manager: Mutex::new(DeviceManager::new().run()),
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
            .service(stats)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
