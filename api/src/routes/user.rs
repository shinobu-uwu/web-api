use crate::{database::get_connection, view_models::user::User};
use ::entity::user;
use rocket::response::status::Accepted;
use rocket::serde::json::Json;
use sea_orm::*;

#[get("/", format = "json")]
pub async fn get_users() -> Json<Vec<user::Model>> {
    let db = get_connection().await;

    Json(
        user::Entity::find()
            .all(&db)
            .await
            .unwrap()
    )
}

#[post("/", data = "<user>", format = "json")]
pub async fn create_user(user: Json<User<'_>>) -> Accepted<String> {
    let conn = get_connection().await;
    user::ActiveModel {
        name: Set(user.name.to_owned()),
        email: Set(user.email.to_owned()),
        ..Default::default()
    }
    .save(&conn)
    .await
    .unwrap();

    Accepted(Some("Created user!".to_owned()))
}
