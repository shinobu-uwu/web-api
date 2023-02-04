use rocket_contrib::json;
use rocket_contrib::json::JsonValue;

#[get("/<id>")]
pub fn index(id: i32) -> JsonValue {
    json!(
    {
        "id": id,
        "name": "Test",
        "email": "test@tes.com"
    }
    )
}
