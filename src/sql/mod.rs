// SPDX-FileCopyrightText: 2024 winston <hey@winston.sh>
// SPDX-License-Identifier: AGPL-3.0-only

use diesel::pg::{Pg, PgConnection};
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub mod message;
pub mod models;
pub mod options;
pub mod schema;
pub mod starboard_message;

pub fn get_connection_pool(
    database_url: String,
) -> Result<Pool<ConnectionManager<PgConnection>>, PoolError> {
    Pool::builder().build(ConnectionManager::<PgConnection>::new(database_url))
}

pub fn run_migrations(
    connection: &mut impl MigrationHarness<Pg>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}
