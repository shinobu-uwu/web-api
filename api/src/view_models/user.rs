use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User<'r> {
    pub name: &'r str,
    pub email: &'r str,
}

impl<'r> User<'r> {
    pub fn new(name: &'r str, email: &'r str) -> Self {
        Self { name, email }
    }
}
