#[macro_use]
extern crate rocket;
use entity::user_register_cache::ActiveModel;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{
    fairing::{self, AdHoc},
    http::Status,
    Build, Rocket,
};

use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};

use migration::MigratorTrait;
use sea_orm::{entity::*, DatabaseConnection};

mod pool;
use pool::Db;
use sea_orm_rocket::{Connection, Database};

pub use entity::*;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, Transport};
use lettre::transport::sendmail::SendmailTransport;


/// # API 테스트
#[openapi(tag = "User")]
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// # 유저 리스트
#[openapi(tag = "User")]
#[get("/users")]
async fn users(conn: Connection<'_, Db>) -> Json<usize> {
    let db = conn.into_inner();

    let users: Vec<user::Model> = user::Entity::find()
        .all(db)
        .await
        .expect("could not find post");

    Json(users.len())
}

#[derive(FromForm, JsonSchema, Serialize, Deserialize)]
struct UserForm {
    email: String,
    password: String,
    name: String,
}

/// # 유저 생성
#[openapi(tag = "User")]
#[post("/users", data = "<user_form>")]
async fn sign_up(conn: Connection<'_, Db>, user_form: Json<UserForm>) -> Status {
    let db: &DatabaseConnection = conn.into_inner();

    let request_data: UserForm = user_form.into_inner();

    user::ActiveModel {
        email: Set(request_data.email.to_owned()),
        password: Set(request_data.password.to_owned()),
        name: Set(request_data.name.to_owned()),
        ..Default::default()
    }
    .save(db)
    .await
    .unwrap();

    Status::NoContent
}

#[derive(FromForm, JsonSchema, Serialize, Deserialize)]
struct UserEmailRequest {
    email: String,
}

/// # 유저 이메일 확인 코드 보내기
#[openapi(tag = "User")]
#[post("/users/email", data = "<user_form>")]
async fn sign_up_email(conn: Connection<'_, Db>, user_form: Json<UserEmailRequest>) -> Status {
    let db: &DatabaseConnection = conn.into_inner();

    let request_data: UserEmailRequest = user_form.into_inner();

    let rand_code: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let user_register_cache: ActiveModel = user_register_cache::ActiveModel {
        email: Set(request_data.email.to_owned()),
        code: Set(rand_code),
        ..Default::default()
    }
    .save(db)
    .await
    .unwrap();

    let email: Message = Message::builder()
        .from("group@group.com".parse().unwrap())
        .to(user_register_cache.email.unwrap().parse().unwrap())
        .subject("Group 회원가입 코드")
        .body(user_register_cache.code.unwrap())
        .unwrap();

    let mailer: SendmailTransport = SendmailTransport::new();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }

    Status::NoContent
}

#[derive(FromForm, JsonSchema, Serialize, Deserialize)]
struct CheckEmailCodeRequest {
    email: String,
    code: String
}
/// # 유저 이메일 코드 확인
#[openapi(tag = "User")]
#[post("/users/email/code", data = "<user_form>")]
async fn check_email_code(conn: Connection<'_, Db>, user_form: Json<CheckEmailCodeRequest>) -> Status {
    let db: &DatabaseConnection = conn.into_inner();

    let request_data: CheckEmailCodeRequest = user_form.into_inner();


    Status::NoContent
}

// #[derive(Serialize)]
// #[serde(crate = "rocket::serde")]
// struct User {
//     email: String,
//     name: String,
//  }

// #[post("/users/login", data = "<user_form>")]
// async fn login(conn: Connection<'_, Db>, user_form: Form<user::Model>) -> &'static str {
//     let db = conn.into_inner();

//     let user: Option<user::Model> = user::Entity::find()
//         .filter(user::Column::Email.eq(&*user_form.email))
//         .filter(user::Column::Password.eq(&*user_form.password))
//         .one(db)
//         .await
//         .expect("could not insert post");

//     match user {
//         Some(user::Model { id, email, name, password }) => "OK",
//         None => panic!("no_user")
//     }
// }

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
        .mount(
            "/",
            openapi_get_routes![index, users, sign_up_email, sign_up],
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
}
