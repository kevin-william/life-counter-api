use actix_web::{test, web, App};
use actix_cors::Cors;
use dotenv::dotenv;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use life_counter_api::handlers;
use life_counter_api::db::DbPool;

pub fn create_test_pool() -> DbPool {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set for tests");
    
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .max_size(1) // Limite para testes
        .build(manager)
        .expect("Failed to create test pool")
}

pub async fn create_test_app(pool: DbPool) -> impl actix_web::dev::Service<
    actix_http::Request,
    Response = actix_web::dev::ServiceResponse,
    Error = actix_web::Error,
> {
    let cors = Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header();

    test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .configure(handlers::contadores::configure)
            .configure(handlers::utilizadores::configure)
            .configure(handlers::contador_utilizadores::configure)
    ).await
}
