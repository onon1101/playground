#[macro_use] extern crate rocket;

use std::env;
use dotenvy::dotenv;
use sqlx::PgPool;

async fn init_db() -> PgPool{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url).await.expect("Failed to connect to database")
}

#[get("/hello")]
fn hello<'a>() -> &'a str {
    "test"
}
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    let db_pool = init_db().await;
    rocket::build()
        .manage(db_pool)
        .mount("/", routes![index])

}