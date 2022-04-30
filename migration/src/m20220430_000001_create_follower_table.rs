use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220430_000001_create_follower_table"
    }
}

#[derive(sea_query::Iden)]
pub enum Follower {
    Table,
    Id,
    FollowingId,
    FollowedId,
    Enable,
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
                    .table(Follower::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Follower::Id)
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Follower::FollowedId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Follower::FollowingId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Follower::Enable)
                            .boolean()
                            .default(true)
                            .not_null(),
                    )
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("fk_following_id_follower")
                            .from(Follower::Table, Follower::FollowingId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("fk_followed_id_follower")
                            .from(Follower::Table, Follower::FollowedId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Follower::Table).to_owned())
            .await
    }
}
