use actix_web::web;

use super::flight::{get_flights, get_flight_by_id, create_flight, update_flight, delete_flight, };

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(get_flights)
        .service(get_flight_by_id)
        .service(create_flight)
        .service(update_flight)
        .service(delete_flight);

    conf.service(scope);
}
