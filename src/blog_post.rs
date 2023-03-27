use crate::error::Error;

use crate::DB;

use actix_web::web::{Json, Path};
use actix_web::{delete, get, post, put};
use serde::Deserialize;
use serde::Serialize;


const BLOG_POST: &str = "blog_post";

#[derive(Serialize, Deserialize)]
pub struct BlogPost {
    title : String
}

#[post("/blog/{id}")]
pub async fn create(id : Path<String>, post: Json<BlogPost>) -> Result<Json<BlogPost>, Error> {
	let post = DB.create((BLOG_POST, &*id)).content(post).await?;
	Ok(Json(post))
}

#[get("/blog/{id}")]
pub async fn read(id: Path<String>) -> Result<Json<Option<BlogPost>>, Error> {
    let post = DB.select((BLOG_POST, &*id)).await?;
    Ok(Json(post))
}

#[put("/blog/{id}")]
pub async fn update(id: Path<String>, post: Json<BlogPost>) -> Result<Json<BlogPost>, Error> {
    let post = DB.update((BLOG_POST, &*id)).content(post).await?;
    Ok(Json(post))
}

#[delete("/blog/{id}")]
pub async fn delete(id: Path<String>) -> Result<Json<()>, Error> {
    DB.delete((BLOG_POST, &*id)).await?;
    Ok(Json(()))
}

#[get("/blog")]
pub async fn list() -> Result<Json<Vec<BlogPost>>, Error> {
    let posts = DB.select(BLOG_POST).await?;
    Ok(Json(posts))
}