#[get("/")]
pub fn index() -> &'static str {
    "Welcome to our web app!"
}
