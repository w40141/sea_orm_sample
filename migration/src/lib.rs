pub use sea_schema::migration::prelude::*;

mod m20220416_000001_create_user_table;
mod m20220416_000002_create_post_table;
mod m20220416_000003_create_favorite_table;
mod m20220430_000001_create_follower_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220416_000001_create_user_table::Migration),
            Box::new(m20220416_000002_create_post_table::Migration),
            Box::new(m20220416_000003_create_favorite_table::Migration),
            Box::new(m20220430_000001_create_follower_table::Migration),
        ]
    }
}
