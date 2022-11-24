use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_user_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
             Table::create()
                .table(User::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(User::Id)
                        .big_unsigned()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                )
                .col(ColumnDef::new(User::Email).string().not_null().unique_key())
                .col(ColumnDef::new(User::Name).string().not_null())
                .col(ColumnDef::new(User::Password).string().not_null())
                .col(ColumnDef::new(User::UpdatedAt).timestamp().not_null().extra("DEFAULT CURRENT_TIMESTAMP".to_owned()))
                .col(ColumnDef::new(User::CreatedAt).timestamp().not_null().extra("DEFAULT CURRENT_TIMESTAMP".to_owned()))
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Email,
    Name,
    Password,
    UpdatedAt,
    CreatedAt,
}