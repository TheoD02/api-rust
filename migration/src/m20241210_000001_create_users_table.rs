use sea_orm_migration::{prelude::*, schema::*};

/// Migration: Create users table
/// Equivalent de: bin/console doctrine:migrations:generate + ecriture du SQL
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // up() = ce qui se passe quand on applique la migration
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_auto(Users::Id))                    // id INT AUTO_INCREMENT PRIMARY KEY
                    .col(string(Users::Username))               // username VARCHAR(255) NOT NULL
                    .col(string_uniq(Users::Email))             // email VARCHAR(255) NOT NULL UNIQUE
                    .col(timestamp(Users::CreatedAt))           // created_at TIMESTAMP NOT NULL
                    .to_owned(),
            )
            .await
    }

    // down() = ce qui se passe quand on rollback (doctrine:migrations:execute --down)
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

// Definition des colonnes (comme les attributs de ton Entity Doctrine)
#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Email,
    CreatedAt,
}
