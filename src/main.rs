#[macro_use]
extern crate rocket;
use rocket::{Rocket, Build, fairing::{self, AdHoc}, serde::Deserialize};
use rocket::form::{Context, Form};

use rocket::serde::{Serialize, json::Json};

use migration::{MigratorTrait, tests_cfg::json};
use sea_orm::{entity::*, QueryFilter};

mod pool;
use pool::Db;
use sea_orm_rocket::{Database, Connection};

pub use entity::*;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/users")]
async fn users(conn: Connection<'_, Db>) -> Json<usize> {
    let db = conn.into_inner();

    let users: Vec<user::Model> = user::Entity::find().all(db).await.expect("could not find post");

    Json(users.len())
}

#[post("/users", data = "<user_form>")]
async fn sign_up(conn: Connection<'_, Db>, user_form: Form<user::Model>) -> &'static str {
    let db = conn.into_inner();

    let form = user_form.into_inner();

    user::ActiveModel {
        email: Set(form.email.to_owned()),
        password: Set(form.password.to_owned()),
        name: Set(form.name.to_owned()),
        ..Default::default()
    }
    .save(db)
    .await
    .expect("could not insert post");

    "OK"
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct UserEmailRequest<'r> {
    email: &'r str,
}

#[post("/users/email", data = "<user_form>")]
async fn sign_up_email(conn: Connection<'_, Db>, user_form: Json<UserEmailRequest<'_>>) -> &'static str {
    let db = conn.into_inner();

    let form = user_form.into_inner();

    user_register_cache::ActiveModel {
        email: Set(form.email.to_owned()),
        code: Set("COCOCODE".to_owned()),
        ..Default::default()
    }.save(db)
    .await
    .unwrap();

    "OK"
}


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct User { 
    email: String,
    name: String,
 }

#[post("/users/login", data = "<user_form>")]
async fn login(conn: Connection<'_, Db>, user_form: Form<user::Model>) -> &'static str {
    let db = conn.into_inner();

    let user: Option<user::Model> = user::Entity::find()
        .filter(user::Column::Email.eq(&*user_form.email))
        .filter(user::Column::Password.eq(&*user_form.password))
        .one(db)
        .await
        .expect("could not insert post");

    match user {
        Some(user::Model { id, email, name, password }) => "OK",
        None => panic!("no_user")
    }
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
        .mount("/", routes![index, users, sign_up, sign_up_email, login])
}
