extern crate dotenv;

use actix_web::{get, App, HttpResponse, HttpRequest, Responder, HttpServer};
use actix_web::http::header;

use urlencoding::decode;

use regex::Regex;

use dotenv::dotenv;
use std::env;

#[get("/")]
async fn search(request: HttpRequest) -> impl Responder {
    let ddg_bang_regex = Regex::new(r"!([a-zA-Z]+)\b").unwrap();
    let query = decode(request.query_string()).expect("UTF-8").into_owned();

    let result = if query.starts_with("?") {
        format!("https://duckduckgo.com/?q={}", &query[1..query.len()])
    } else if ddg_bang_regex.is_match(&query) {
        format!("https://duckduckgo.com/?q={}", &query)
    } else {
        format!("https://duckduckgo.com/?q=\\{}", &query)
    };

    HttpResponse::TemporaryRedirect()
        .header(header::LOCATION, result)
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = env::var("HOST").unwrap_or(String::from("127.0.0.1"));
    let port = env::var("PORT").unwrap_or(String::from("8080"));

    let server = HttpServer::new(|| 
        App::new()
            .service(search)
    )
    .bind(format!("{}:{}", &host, &port))?
    .run();

    println!("Server listening on {}:{}", host, port);
    
    server.await
}
