#![feature(proc_macro_hygiene, decl_macro)]

mod routes;

#[macro_use]
extern crate rocket;

fn main() {
    rocket::ignite()
        .mount("/", routes![crate::routes::index::index,])
        .mount("/user", routes![crate::routes::user::index])
        .launch();
}
