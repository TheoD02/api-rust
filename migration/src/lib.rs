pub use sea_orm_migration::prelude::*;

// Liste des migrations (comme le dossier migrations/ en Doctrine)
mod m20241210_000001_create_users_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        // Ajoute tes nouvelles migrations ici dans l'ordre chronologique
        vec![
            Box::new(m20241210_000001_create_users_table::Migration),
        ]
    }
}
