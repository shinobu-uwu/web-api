use dotenv::dotenv;
use sea_orm::DatabaseConnection;

pub async fn get_connection() -> DatabaseConnection {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    sea_orm::Database::connect(&database_url).await.unwrap()
}
