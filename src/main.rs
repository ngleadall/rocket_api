// Externals
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::response::status;
use rocket::Build;
use rocket_contrib::json::{Json, JsonValue};

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
embed_migrations!();
#[database("sqlite_path")]
struct DbConn(diesel::SqliteConnection);

// Internals
mod auth;
mod models;
mod respositories;
mod schema;

use auth::BasicAuth; // BasicAuth Struct
use models::*; // User struct
use respositories::*;

// Routes
#[get("/users")]
async fn get_users(_auth: BasicAuth, conn: DbConn) -> Result<JsonValue, status::Custom<JsonValue>> {
    conn.run(|c| {
        UserRepository::load_all(c)
            .map(|users| json!(users))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[get("/users/<id>")]
async fn view_user(
    id: i32,
    _auth: BasicAuth,
    conn: DbConn,
) -> Result<JsonValue, status::Custom<JsonValue>> {
    conn.run(move |c| {
        UserRepository::find(c, id)
            .map(|users| json!(users))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[post("/users", format = "json", data = "<new_user>")]
async fn create_user(
    _auth: BasicAuth,
    conn: DbConn,
    new_user: Json<NewUser>,
) -> Result<JsonValue, status::Custom<JsonValue>> {
    conn.run(|c| {
        UserRepository::create(c, new_user.into_inner())
            .map(|users| json!(users))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[put("/users/<_id>", format = "json", data = "<user>")]
async fn update_user(
    _id: i32,
    _auth: BasicAuth,
    conn: DbConn,
    user: Json<User>,
) -> Result<JsonValue, status::Custom<JsonValue>> {
    conn.run(move |c| {
        UserRepository::save(c, user.into_inner())
            .map(|users| json!(users))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[delete("/rustaceans/<id>")]
async fn delete_user(
    id: i32,
    _auth: BasicAuth,
    conn: DbConn,
) -> Result<status::NoContent, status::Custom<JsonValue>> {
    conn.run(move |c| {
        UserRepository::delete(c, id)
            .map(|_| status::NoContent)
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

// Catchalls
#[catch(404)]
fn not_found() -> JsonValue {
    json!("Resource not found!")
}

#[catch(401)]
fn unathorized() -> JsonValue {
    json!("Not authorized")
}

#[catch(422)]
fn unprocessable() -> JsonValue {
    json!("Invalid entity. Missing fields")
}

// Init migrations
async fn run_db_migrations(
    rocket: rocket::Rocket<Build>,
) -> Result<rocket::Rocket<Build>, rocket::Rocket<Build>> {
    DbConn::get_one(&rocket)
        .await
        .expect("Failed to connect to database")
        .run(|c| match embedded_migrations::run(c) {
            Ok(()) => Ok(rocket),
            Err(e) => {
                println!("Failed to run database migrations: {:?}", e);
                Err(rocket)
            }
        })
        .await
}

// Launch
#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![get_users, view_user, create_user, update_user, delete_user],
        )
        .register("/", catchers![not_found, unathorized, unprocessable])
        .attach(DbConn::fairing())
        .attach(AdHoc::try_on_ignite(
            "Database Migrations",
            run_db_migrations,
        ))
        .launch()
        .await;
}
