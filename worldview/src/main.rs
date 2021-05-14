use std::sync::Arc;
use std::sync::RwLock;

use actix_web::{
    error::InternalError, get, http::StatusCode, post, web::Data, web::Json, web::JsonConfig,
    web::ServiceConfig, HttpResponse, Responder,
};

use damn_vuln_blockchain::payload::Status;

#[actix_web::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> std::io::Result<()> {
    use actix_web::{
        middleware::{normalize, Compress, Logger, NormalizePath},
        App, HttpServer,
    };

    pretty_env_logger::init();

    let data: StakeData = StakeData::default();

    let ip = format!("0.0.0.0:{}", std::env::var("PORT").unwrap());
    let server_fut = HttpServer::new(move || {
        App::new()
            .app_data(get_json_err())
            .data(data.clone())
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(NormalizePath::new(normalize::TrailingSlash::Trim))
            .configure(services)
    })
    .bind(ip)
    .unwrap()
    .run();

    server_fut.await
}

type StakeData = Arc<RwLock<Vec<Status>>>;

#[cfg(not(tarpaulin_include))]
fn get_json_err() -> JsonConfig {
    JsonConfig::default().error_handler(|err, _| {
        //debug!("JSON deserialization error: {:?}", &err);
        InternalError::new(err, StatusCode::BAD_REQUEST).into()
    })
}

#[get("/")]
async fn auditor() -> impl Responder {
    const INDEX: &str = include_str!("../../frontend/index.html");
    HttpResponse::Ok().content_type("text/html").body(INDEX)
}

#[get("/worldview")]
async fn worldview(d: Data<StakeData>) -> impl Responder {
    let state = d.read().unwrap();
    HttpResponse::Ok().json(&*state)
}

#[post("/worldview")]
async fn update(d: Data<StakeData>, payload: Json<Vec<Status>>) -> impl Responder {
    let mut state = d.write().unwrap();
    *state = payload.into_inner();
    drop(state);
    HttpResponse::Ok()
}

pub fn services(cfg: &mut ServiceConfig) {
    cfg.service(worldview);
    cfg.service(update);
    cfg.service(auditor);
}
