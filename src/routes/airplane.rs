use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[get("/planes")]
pub async fn get_plane_info() -> impl Responder{
    const NAME: &str = "Boeing 690";
    HttpResponse::Ok().json(json!({"id":1,"name": NAME}))
}