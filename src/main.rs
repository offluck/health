#[derive(Debug)]
struct Response<'a> {
    status: &'a str,
}

#[actix_web::get("/")]
async fn hello() -> impl actix_web::Responder {
    "hello"
}

#[actix_web::get("/health/")]
async fn health_state() -> impl actix_web::Responder {
    format!("{:?}", Response { status: "OK" })
}

#[actix_web::main]
async fn main() {
    println!("Running!");
    actix_web::HttpServer::new(|| actix_web::App::new().service(hello).service(health_state))
        .bind(("0.0.0.0", 8000))
        .unwrap()
        .run()
        .await
        .unwrap();
}
