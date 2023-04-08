use crate::notes::{Note, Notes};
use crate::error_handler::CustomError;
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/notes")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let notes = web::block(|| Notes::find_all()).await.unwrap();
    Ok(HttpResponse::Ok().json(notes))
}
#[post("/notes")]
async fn create(note: web::Json<Note>) -> Result<HttpResponse, CustomError> {
    let note = Notes::create(note.into_inner())?;
    Ok(HttpResponse::Ok().json(note)) //TODO: ui is expecting a JSON with an "id" field and a "content" field
}
#[put("/notes/{id}")]
async fn update(
    id: web::Path<i32>,
    note: web::Json<Note>,
) -> Result<HttpResponse, CustomError> {
    let note = Notes::update(id.into_inner(), note.into_inner())?;
    Ok(HttpResponse::Ok().json(note)) //TODO: ui is expecting a JSON with an "id" field and a "content" field
}
#[delete("/notes/{id}")]
//the id parameter is extracted from the URL path using the web::Path extractor
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_note = Notes::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_note })))
}
pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(create);
    config.service(update);
    config.service(delete);
}