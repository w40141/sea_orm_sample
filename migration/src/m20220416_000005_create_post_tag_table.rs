use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220416_000005_create_post_tag_table"
    }
}

#[derive(sea_query::Iden)]
pub enum PostTag {
    Table,
    Id,
    PostId,
    TagId,
    CreatedAt,
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(PostTag::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PostTag::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PostTag::PostId).string().not_null())
                    .col(ColumnDef::new(PostTag::TagId).string().not_null())
                    .col(ColumnDef::new(PostTag::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(PostTag::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PostTag::Table).to_owned())
            .await
    }
}
