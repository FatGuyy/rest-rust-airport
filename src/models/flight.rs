use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Flight {
    pub id: Uuid,
    pub flight_name: String,
    pub take_off_location: String,
    pub landing_location: String,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFlightSchema {
    pub flight_name: String,
    pub take_off_location: String,
    pub landing_location: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateFlightSchema {
    pub flight_name: Option<String>,
    pub take_off_location: Option<String>,
    pub landing_location: Option<String>
}

