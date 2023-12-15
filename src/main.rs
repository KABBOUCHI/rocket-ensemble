use models::User;

#[macro_use]
extern crate rocket;

mod migrations;
mod models;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/users/<user>")]
async fn show_user(user: User) -> User {
    user
}

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();

    ensemble::setup(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .expect("Failed to set up database pool.");

    ensemble::migrate!(migrations::CreateUsersTable)
        .await
        .expect("Failed to run migrations.");

    rocket::build().mount("/", routes![index, show_user])
}
