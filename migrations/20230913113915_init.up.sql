-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS flights (
    "id" uuid PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    "plane_name" varchar NOT NULL,
    "flight_name" varchar NOT NULL,
    "day" varchar NOT NULL,
    "Starting_location" TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    "Landing_location" TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);