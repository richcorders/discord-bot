use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub mod message;
pub mod models;
pub mod schema;

pub fn get_connection_pool(
) -> Result<Pool<ConnectionManager<PgConnection>>, Box<dyn std::error::Error + Send + Sync>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Ok(Pool::builder().build(manager)?)
}
