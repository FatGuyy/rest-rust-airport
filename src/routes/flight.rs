use crate::models::flight::{Flight, CreateFlightSchema, UpdateFlightSchema};
use crate::AppState;


// TODO: check if I can delete this imports if also used somewhere else
use actix_web::{get, post, put, web, HttpResponse, Responder, delete};
use chrono::Utc;
use serde_json::json;


#[get("/flights")]
pub async fn get_flights(data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(
        Flight,
        "SELECT * FROM flights"
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let message: &str = "Something bad happened while fetching the flights";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": message}))
    }

    let flights = query_result.unwrap();

    HttpResponse::Ok().json(json!({
        "status": "success",
        "no. flights": flights.len(),
        "flights": flights
    }))
}

#[post("/flights/flight")]
async fn create_flight(body: web::Json<CreateFlightSchema>, data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(
        Flight,
        "INSERT into flights (flight_name, take_off_location, landing_location) values ($1, $2, $3) returning *",
        body.flight_name.to_string(),
        body.take_off_location.to_string(),
        body.landing_location.to_string()
    ).fetch_one(&data.db)
    .await;

    match query_result {
        Ok(flight) => {
            let response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "flight": flight
            })});
            return HttpResponse::Ok().json(response);
        }
        Err(e) => {
            if e.to_string().contains("duplicate key value violates unique constraint") {
                return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "fail", "message": "Duplicate Key"}))
            }
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": format!("{:?}", e)}));
        }
    }
}

#[get("/flights/flight/{id}")]
async fn get_flight_by_id(path: web::Path<uuid::Uuid>, data: web::Data<AppState>) -> impl Responder {
    let flight_id = path.into_inner();
    let query_result = sqlx::query_as!(Flight, "SELECT * FROM flights WHERE id = $1", flight_id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(flight) => {
            let response = serde_json::json!({"status": "success", "data": serde_json::json!({
                "flight": flight
            })});
            return HttpResponse::Ok().json(response);
        }
        Err(_) => {
            let message = format!("Flight with ID: {} not found", flight_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail", "message": message}));
        }
    }
}

#[put("/flights/flight/{id}")]
async fn update_flight(path: web::Path<uuid::Uuid>, data: web::Data<AppState>, body: web::Json<UpdateFlightSchema>) -> impl Responder {
    let flight_id = path.into_inner();
    // make sure flight exists before updating
    let query_result = sqlx::query_as!(Flight, "SELECT * FROM flights where id = $1", flight_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let message = format!("Flight with ID: {} not found", flight_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail", "message": message}));
    }

    let now = Utc::now();
    let flight = query_result.unwrap();

    let query_result = sqlx::query_as!(
        Flight,
        "UPDATE flights set flight_name = $1, take_off_location = $2, landing_location = $3, updated_at = $4 where id = $5 returning *",
        body.flight_name.to_owned().unwrap_or(flight.flight_name),
        body.take_off_location.to_owned().unwrap_or(flight.take_off_location),
        body.landing_location.to_owned().unwrap_or(flight.landing_location),
        now,
        flight_id
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(flight) => {
            let response = serde_json::json!({"state": "success", "data": serde_json::json!({
                "flight": flight
            })});
            return HttpResponse::Ok().json(response);
        }
        Err (_) => {
            let message = format!("Flight with ID: {} not found", flight_id);
            return HttpResponse::NotFound()
                .json(serde_json::json!({"status": "fail", "message": message}))
        }
    }
}

#[delete("/flights/flight/{id}")]
async fn delete_flight(path: web::Path<uuid::Uuid>, data: web::Data<AppState>) -> impl Responder {
    let flight_id = path.into_inner();
    let rows_affected = sqlx::query!("DELETE from flights WHERE id = $1", flight_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let message = format!("Flight with ID: {} not found", flight_id);
        return HttpResponse::NotFound().json(json!({"status": "fail", "message": message}))
    }
    HttpResponse::NoContent().finish()
}
