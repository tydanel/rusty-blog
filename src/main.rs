mod blog_post;
mod error;

use actix_web;
use actix_web::{App, HttpServer};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;




static DB: Surreal<Client> = Surreal::init();

#[actix_web::main] // or #[tokio::main]
async fn main() -> Result<(), Box< dyn std::error::Error>> {
    DB.connect::<Ws>("localhost:8000").await?;

    DB.signin(Root {
        username: "root",
        password: "root",
    }).await?;
    

    DB.use_ns("namespace").use_db("database").await?;
    
    println!("Database connected");

    HttpServer::new(|| {
        App::new()
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