use chrono::Utc;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Plane{
    pub id:Uuid,
    pub plane_name: String,
    pub plane_model: String,
    pub capacity: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Flight{
    pub id:Uuid,
    pub plane_name: String,
    pub flight_name: String,
    pub Starting_location: String,
    pub Landing_location: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFlightSchema {
    pub flight_name: String,
    pub plane_name: String,
    pub Starting_location: String,
    pub Landing_location: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateflightSchema {
    pub flight_name: Option<String>,
    pub plane_name: String,
    pub Starting_location: Option<String>,
    pub Landing_location: Option<String>,
}

