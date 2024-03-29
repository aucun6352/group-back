use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(UserRegisterCache::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(UserRegisterCache::Id)
                        .big_unsigned()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(UserRegisterCache::Email).string().not_null())
                .col(ColumnDef::new(UserRegisterCache::Code).string())
                .col(ColumnDef::new(UserRegisterCache::UpdatedAt).timestamp().not_null().extra("DEFAULT CURRENT_TIMESTAMP".to_owned()))
                .col(ColumnDef::new(UserRegisterCache::CreatedAt).timestamp().not_null().extra("DEFAULT CURRENT_TIMESTAMP".to_owned()))
                .to_owned(),
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserRegisterCache::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum UserRegisterCache {
    Table,
    Id,
    Email,
    Code,
    UpdatedAt,
    CreatedAt
}
