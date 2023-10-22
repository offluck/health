#[derive(Debug)]
struct Response<'a> {
    status: &'a str,
}

#[actix_web::get("/health/")]
async fn get_health_state() -> impl actix_web::Responder {
    format!("{:?}", Response { status: "OK" })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| actix_web::App::new().service(get_health_state))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
