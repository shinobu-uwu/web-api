#![feature(proc_macro_hygiene, decl_macro)]
use database::get_connection;
use migration::{Migrator, MigratorTrait};

mod database;
mod routes;
mod view_models;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

#[tokio::main]
async fn main() {
    let connection = get_connection();
    Migrator::up(&connection.await, None).await.unwrap();

    rocket::build()
        .mount("/", routes![crate::routes::index::index,])
        .mount("/users", routes![crate::routes::user::create_user, crate::routes::user::get_users])
        .launch()
        .await
        .unwrap();
}
