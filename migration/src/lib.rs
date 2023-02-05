pub use sea_orm_migration::prelude::*;

#[allow(non_snake_case)]
mod m20230204_151610_Initial;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230204_151610_Initial::Migration),
        ]
    }
}
