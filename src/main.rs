mod blog_post;
mod error;

use actix_web;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use env_logger::Env;

static DB: Surreal<Client> = Surreal::init();

#[actix_web::main] // or #[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    DB.connect::<Ws>("localhost:8000").await?;

    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    DB.use_ns("namespace").use_db("database").await?;

    println!("Database connected");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(blog_post::create)
            .service(blog_post::read)
            .service(blog_post::update)
            .service(blog_post::delete)
            .service(blog_post::list)
    })
    .bind(("localhost", 8080))?
    .run()
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::{header::ContentType},
        test
    };


    #[actix_web::test]
    async fn test_index_not_found() {
        let app = test::init_service(
            App::new()
                .service(blog_post::list)
        ).await;
        
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();

        let resp = test::call_service(&app, req).await;
        
        assert!(resp.status().is_client_error());

    }
}
