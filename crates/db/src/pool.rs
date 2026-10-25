/* db/src/pool.rs
 * The purpose of this module is to serve the database pool.
 *
 * Copyright (C) 2026  https://github.com/brandongrahamcobb/Quit.git
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::error::DatabaseError;
use config::settings;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub async fn spawn_db<F, T>(pool: &DbPool, f: F) -> Result<T, DatabaseError>
where
    F: FnOnce(&mut PgConnection) -> Result<T, diesel::result::Error> + Send + 'static,
    T: Send + 'static,
{
    let pool = pool.clone();
    tokio::task::spawn_blocking(move || {
        let mut conn = pool.get().map_err(|e| {
            diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            )
        })?;
        f(&mut conn)
    })
    .await
    .map_err(DatabaseError::JoinError)?
    .map_err(DatabaseError::from)
}

pub fn new() -> Result<DbPool, DatabaseError> {
    let db_url = settings::get_db_url()?;
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let db = Pool::builder()
        .build(manager)
        .map_err(DatabaseError::DatabaseConnectionError)?;
    Ok(db)
}
