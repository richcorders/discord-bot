use std::env;

use diesel::pg::{Pg, PgConnection};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub mod message;
pub mod models;
pub mod schema;

pub fn get_connection_pool(
) -> Result<Pool<ConnectionManager<PgConnection>>, Box<dyn std::error::Error + Send + Sync>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Ok(Pool::builder().build(manager)?)
}

pub fn run_migrations(
    connection: &mut impl MigrationHarness<Pg>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}
