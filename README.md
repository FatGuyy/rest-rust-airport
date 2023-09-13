# rest-rust-airport
### REST API created in Rust using Acix web 4, postgress database. 

## API Routes
### Get All Flights

- **URL:** `/flights`
- **HTTP Method:** `GET`
- **Description:** Retrieve a list of all flights.
- **Response:**
  - Status Code: `200 OK`
  - JSON Response Example:
    ```json
    {
        "status": "success",
        "number_of_flights": 3,
        "flights": [
            {
                "id": "d02f10f6-64b3-4cf2-ae68-76d4634e551b",
                "flight_name": "Flight 1",
                "plane_name": "Plane A",
                "Starting_location": "Location A",
                "Landing_location": "Location B"
            },
            // Other flight objects...
        ]
    }
    ```

### Get Flight by ID

- **URL:** `/flights/{id}`
- **HTTP Method:** `GET`
- **Description:** Retrieve a specific flight by its unique ID.
- **URL Parameter:**
  - `{id}`: The UUID of the flight.
- **Response:**
  - Status Code: `200 OK` if the flight is found.
  - Status Code: `404 Not Found` if the flight with the given ID doesn't exist.
  - JSON Response Example (Success):
    ```json
    {
        "status": "success",
        "flight": {
            "id": "d02f10f6-64b3-4cf2-ae68-76d4634e551b",
            "flight_name": "Flight 1",
            "plane_name": "Plane A",
            "Starting_location": "Location A",
            "Landing_location": "Location B"
        }
    }
    ```
  - JSON Response Example (Not Found):
    ```json
    {
        "status": "fail",
        "message": "Flight with ID: {id} not found"
    }
    ```

### Create a New Flight

- **URL:** `/flights`
- **HTTP Method:** `POST`
- **Description:** Create a new flight with the provided data.
- **Request Body:**
  - JSON Object with flight details (e.g., `"flight_name"`, `"plane_name"`, `"Starting_location"`, `"Landing_location"`).
- **Response:**
  - Status Code: `201 Created` if the flight is created successfully.
  - Status Code: `500 Internal Server Error` if the creation fails.
  - JSON Response Example (Success):
    ```json
    {
        "status": "success",
        "message": "Flight created successfully",
        "flight": {
            "id": "d02f10f6-64b3-4cf2-ae68-76d4634e551b",
            "flight_name": "Flight 1",
            "plane_name": "Plane A",
            "Starting_location": "Location A",
            "Landing_location": "Location B"
        }
    }
    ```

### Update a Flight

- **URL:** `/flights/{id}`
- **HTTP Method:** `PUT`
- **Description:** Update an existing flight with the provided data.
- **URL Parameter:**
  - `{id}`: The UUID of the flight to update.
- **Request Body:**
  - JSON Object with updated flight details (e.g., `"flight_name"`, `"plane_name"`, `"Starting_location"`, `"Landing_location"`).
- **Response:**
  - Status Code: `200 OK` if the flight is updated successfully.
  - Status Code: `404 Not Found` if the flight with the given ID doesn't exist.
  - JSON Response Example (Success):
    ```json
    {
        "status": "success",
        "message": "Flight updated successfully",
        "flight": {
            "id": "d02f10f6-64b3-4cf2-ae68-76d4634e551b",
            "flight_name": "Updated Flight 1",
            "plane_name": "Updated Plane A",
            "Starting_location": "Updated Location A",
            "Landing_location": "Updated Location B"
        }
    }
    ```

### Delete a Flight

- **URL:** `/flights/{id}`
- **HTTP Method:** `DELETE`
- **Description:** Delete a flight by its unique ID.
- **URL Parameter:**
  - `{id}`: The UUID of the flight to delete.
- **Response:**
  - Status Code: `204 No Content` if the flight is deleted successfully.
  - Status Code: `404 Not Found` if the flight with the given ID doesn't exist.

Please note that you should replace `{id}` in the route URLs with the actual UUID of the flight you want to access or manipulate.
