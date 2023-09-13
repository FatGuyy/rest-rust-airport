// mod routes;
// mod models;

use actix_cors::Cors;
use actix_web::{http::header, HttpServer, middleware::Logger, App};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use dotenv::dotenv;
// use routes::{config::config};

pub struct AppState{
    db: Pool<Postgres>,
} 

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    if std::env::var_os("RUST_LOG").is_none(){
        std::env::set_var("RUST_LOG", "actix_web=info")
    }
    dotenv().ok();
    env_logger::init();

    let database_url:String = std::env::var("DATABASE_URL").expect("DATABASE URL is not set ");
    let pool:Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to db is successful! ✅");
            pool
        }
        Err(err) => {
            println!("Failed to connect to db ❌");
            std::process::exit(1);
        }
    };
    print!("Server started successfully! 🔥");

    HttpServer::new(move || {
        let cors = Cors::default()
          .allowed_origin("http:://localhost::8080")
          .allowed_methods(vec!["GET","POST", "PUT", "DELETE"])
          .allowed_origin("http://localhost:8080")
          .allowed_headers(vec![
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::CONTENT_ENCODING,
            header::ACCEPT,
          ])
          .supports_credentials();
        App::new()
            .app_data(actix_web::web::Data::new(AppState{db:pool.clone()}))
            // .service(health_checker_handler)
            // .configure(config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}
