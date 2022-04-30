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

#[derive(sea_query::Iden)]
pub enum Post {
    Table,
    Id,
}

#[derive(sea_query::Iden)]
pub enum User {
    Table,
    Id,
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
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Favorite::UserId).big_unsigned().not_null())
                    .col(ColumnDef::new(Favorite::PostId).big_unsigned().not_null())
                    .col(ColumnDef::new(Favorite::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Favorite::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("fk_user_id_favorite")
                            .from(Favorite::Table, Favorite::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("fk_post_id_favorite")
                            .from(Favorite::Table, Favorite::PostId)
                            .to(Post::Table, Post::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
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
