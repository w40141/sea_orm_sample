pub use sea_schema::migration::prelude::*;

mod m20220416_000001_create_user_table;
mod m20220416_000002_create_post_table;
mod m20220416_000003_create_favorite_table;
mod m20220416_000004_create_tag_table;
mod m20220416_000005_create_post_tag_table;
mod m20220417_000001_drop_password_index;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220416_000001_create_user_table::Migration),
            Box::new(m20220416_000002_create_post_table::Migration),
            Box::new(m20220416_000003_create_favorite_table::Migration),
            Box::new(m20220416_000004_create_tag_table::Migration),
            Box::new(m20220416_000005_create_post_tag_table::Migration),
            Box::new(m20220417_000001_drop_password_index::Migration),
        ]
    }
}
