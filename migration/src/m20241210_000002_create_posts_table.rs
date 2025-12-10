use sea_orm_migration::{prelude::*, schema::*};

/// Migration: Create posts table with JSON metadata
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Posts::Table)
                    .if_not_exists()
                    .col(pk_auto(Posts::Id))
                    .col(string(Posts::Title))
                    .col(text(Posts::Content))
                    .col(integer(Posts::AuthorId))
                    // JSON field pour stocker des données imbriquées (tags, metadata)
                    .col(json(Posts::Metadata))
                    .col(boolean(Posts::Published).default(false))
                    .col(timestamp(Posts::CreatedAt))
                    .col(timestamp_null(Posts::UpdatedAt))
                    // Foreign key vers users
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_posts_author")
                            .from(Posts::Table, Posts::AuthorId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Posts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
    Title,
    Content,
    AuthorId,
    Metadata,
    Published,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
