use crate::AppState;
use crate::models::airport::{Flight, CreateFlightSchema, UpdateflightSchema};

use sqlx;
use sqlx::types::Uuid;
use actix_web::{get, post, put, web, HttpResponse, Responder, delete};
use serde_json::json;

// GET all flights
#[get("/flights")]
pub async fn get_flights(data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(
        Flight,
        "SELECT * FROM flights",
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": "Something went wrong!"}));
    }

    let flights = query_result.unwrap();
    let json_response = serde_json::json!({
        "status": "success",
        "number_of_flights": flights.len(),
        "flights": flights
    });

    HttpResponse::Ok().json(json_response)
}

// GET a flight by ID
#[get("/flights/{id}")]
pub async fn get_flight_by_id(path: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
    let flight_id = path.into_inner();
    let query_result = sqlx::query_as!(Flight, "SELECT * FROM flights WHERE id = $1", flight_id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(flight) => {
            let json_response = serde_json::json!({
                "status": "success",
                "flight": flight
            });
            HttpResponse::Ok().json(json_response)
        }
        Err(_) => {
            let message = format!("Flight with ID: {} not found", flight_id);
            HttpResponse::NotFound().json(json!({"status": "fail", "message": message}))
        }
    }
}

// POST a new flight
#[post("/flights")]
pub async fn create_flight(body: web::Json<CreateFlightSchema>, data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query!(
        "INSERT INTO flights (flight_name, plane_name, Starting_location, Landing_location) VALUES ($1, $2, $3, $4) RETURNING *",
        body.flight_name.clone(),
        body.plane_name.clone(),
        body.Starting_location.clone(),
        body.Landing_location.clone(),
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(new_flight) => {
            let json_response = serde_json::json!({
                "status": "success",
                "message": "Flight created successfully",
                "flight": new_flight
            });
            HttpResponse::Created().json(json_response)
        }
        Err(_) => {
            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to create flight"}))
        }
    }
}

// PUT (update) a flight by ID
#[put("/flights/{id}")]
pub async fn update_flight(
    path: web::Path<Uuid>,
    body: web::Json<UpdateFlightSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let flight_id = path.into_inner();
    let query_result = sqlx::query!(
        "UPDATE flights SET flight_name = $1, plane_name = $2, Starting_location = $3, Landing_location = $4 WHERE id = $5 RETURNING *",
        body.flight_name.clone(),
        body.plane_name.clone(),
        body.Starting_location.clone(),
        body.Landing_location.clone(),
        flight_id,
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(updated_flight) => {
            let json_response = serde_json::json!({
                "status": "success",
                "message": "Flight updated successfully",
                "flight": updated_flight
            });
            HttpResponse::Ok().json(json_response)
        }
        Err(_) => {
            let message = format!("Flight with ID: {} not found", flight_id);
            HttpResponse::NotFound().json(json!({"status": "fail", "message": message}))
        }
    }
}

// DELETE a flight by ID
#[delete("/flights/{id}")]
pub async fn delete_flight(
    path: web::Path<Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let flight_id = path.into_inner();
    let rows_affected = sqlx::query!(
        "DELETE FROM flights WHERE id = $1",
        flight_id,
    )
    .execute(&data.db)
    .await
    .unwrap()
    .rows_affected();

    if rows_affected > 0 {
        HttpResponse::NoContent().finish()
    } else {
        let message = format!("Flight with ID: {} not found", flight_id);
        HttpResponse::NotFound().json(json!({"status": "fail", "message": message}))
    }
}
