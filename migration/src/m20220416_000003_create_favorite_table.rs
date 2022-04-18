use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220416_000003_create_favorite_table"
    }
}

#[derive(sea_query::Iden)]
pub enum Favorite {
    Table,
    Id,
    UserId,
    PostId,
    CreatedAt,
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(Favorite::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Favorite::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Favorite::UserId).string().not_null())
                    .col(ColumnDef::new(Favorite::PostId).string().not_null())
                    .col(
                        ColumnDef::new(Favorite::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Favorite::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Favorite::Table).to_owned())
            .await
    }
}
