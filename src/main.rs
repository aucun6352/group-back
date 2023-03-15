#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{
    fairing::{self, AdHoc},
    http::Status,
    Build, Rocket,
};

use migration::MigratorTrait;
use sea_orm::{entity::*, DatabaseConnection, QueryFilter};

mod pool;
use pool::Db;
use sea_orm_rocket::{Connection, Database};

pub use entity::*;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use rocket_okapi::{openapi, openapi_get_routes, JsonSchema};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use lettre::transport::sendmail::SendmailTransport;
use lettre::{Message, Transport};

mod ex_attr;

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
    /// 유저 이메일
    #[schemars(example = "ex_attr::email")]
    email: String,
    /// 인증 코드
    #[schemars(example = "ex_attr::code")]
    code: String,
    /// 계정 비밀번호
    password: String,
    /// 유저 이름
    #[schemars(example = "ex_attr::user_name")]
    name: String,
}

/// # 유저 생성
#[openapi(tag = "User")]
#[post("/users", data = "<user_form>")]
async fn sign_up(conn: Connection<'_, Db>, user_form: Json<UserForm>) -> Status {
    let db: &DatabaseConnection = conn.into_inner();

    let request_data: UserForm = user_form.into_inner();

    let code_cache = user_register_cache::Entity::find()
        .filter(user_register_cache::Column::Code.eq(request_data.code))
        .filter(user_register_cache::Column::Email.eq(request_data.email.to_owned()))
        .one(db)
        .await
        .unwrap();

    // code_cache.exists();

    if let None = code_cache {
        return Status::BadRequest;
    }

    user::ActiveModel {
        email: Set(request_data.email.to_owned()),
        name: Set(request_data.name.to_owned()),
        password: Set(request_data.password.to_owned()),
        ..Default::default()
    }
    .save(db)
    .await
    .unwrap();

    Status::Created
}

#[derive(FromForm, JsonSchema, Serialize, Deserialize)]
struct UserEmailRequest {
    #[schemars(example = "ex_attr::email")]
    email: String,
}

/// # 유저 이메일 확인 코드 보내기
#[openapi(tag = "User")]
#[post("/users/validate_email", data = "<user_form>")]
async fn sign_up_email(conn: Connection<'_, Db>, user_form: Json<UserEmailRequest>) -> Status {
    let db: &DatabaseConnection = conn.into_inner();

    let request_data: UserEmailRequest = user_form.into_inner();

    let rand_code: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let user_register_cache = user_register_cache::ActiveModel {
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
    #[schemars(example = "ex_attr::email")]
    email: String,
    #[schemars(example = "ex_attr::code")]
    code: String,
}
/// # 유저 이메일 코드 확인
#[openapi(tag = "User")]
#[post("/users/validate_code", data = "<user_form>")]
async fn check_email_code(
    conn: Connection<'_, Db>,
    user_form: Json<CheckEmailCodeRequest>,
) -> Status {
    let db: &DatabaseConnection = conn.into_inner();

    let request_data: CheckEmailCodeRequest = user_form.into_inner();

    // let is_exists_email = user_register_cache::Entity::find()
    //     .filter(user_register_cache::Column::Code.eq(request_data.code))
    //     .filter(user_register_cache::Column::Email.eq(request_data.email))
    //     .query().limit(1).clear_selects();

    let code_cache = user_register_cache::Entity::find()
        .filter(user_register_cache::Column::Code.eq(request_data.code))
        .filter(user_register_cache::Column::Email.eq(request_data.email))
        .one(db)
        .await
        .unwrap()
        .is_some();

    match code_cache {
        true => Status::Ok,
        false => Status::BadRequest,
    }

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
            openapi_get_routes![index, users, sign_up_email, sign_up, check_email_code],
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}
