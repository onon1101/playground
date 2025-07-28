#[macro_use] extern crate rocket;

use std::env;
use dotenvy::dotenv;
use rocket_okapi::{openapi, openapi_get_routes};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use sqlx::PgPool;

async fn init_db() -> PgPool{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url).await.expect("Failed to connect to database")
}

#[openapi]
#[get("/hello")]
fn hello() -> &'static str {
    "test"
}

#[openapi]
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    let db_pool = init_db().await;
    rocket::build()
        .manage(db_pool)
        .mount("/", openapi_get_routes![hello, index])
        .mount("/swagger", make_swagger_ui(&SwaggerUIConfig {
            url: "/openapi.json".to_string(),
            ..Default::default()
        }))
}