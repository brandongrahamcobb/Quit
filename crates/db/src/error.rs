/* db/error.rs
 * The purpose of this module is to provide errors related to the database.
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

use config::error::ConfigError;
use diesel;
use r2d2;
use thiserror::Error;
use tokio::task::JoinError;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Failed to connect to database in database layer")]
    DatabaseConnectionError(#[from] r2d2::Error),

    #[error("Invalid database url error: {0}")]
    InvalidDatabaseUrlError(String),

    #[error("Configuration error in database layer")]
    ConfigError(#[from] ConfigError),

    #[error("Entry in database not found in database layer")]
    DieselError(#[from] diesel::result::Error),

    #[error("Concurrency join error in database layer")]
    JoinError(#[from] JoinError),
}
