use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    let figment = rocket::Config::figment();
    let database_url: String = figment
        .extract_inner("databases.sea_orm.url")
        .expect("Cannot find Database URL in Rocket.toml");
    std::env::set_var("DATABASE_URL", database_url);

    cli::run_cli(migration::Migrator).await;
}
