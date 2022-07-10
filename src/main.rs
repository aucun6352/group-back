#[macro_use]
extern crate rocket;
use rocket::{Rocket, Build, fairing::{self, AdHoc}};

use migration::MigratorTrait;
use sea_orm::{entity::*};

mod pool;
use pool::Db;
use sea_orm_rocket::{Database, Connection};

pub use entity::*;

use rocket::serde::json::Json;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/users")]
async fn users(conn: Connection<'_, Db>) -> Json<usize> {
    let db = conn.into_inner();

    let post: Vec<user::Model> = user::Entity::find().all(db).await.expect("could not find post");

    Json(post.len())
}


async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/", routes![index, users])
}
