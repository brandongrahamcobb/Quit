/* inc/src/error.rs
 * The purpose of this module is to provide errors related to helper methods.
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

use bcrypt::BcryptError;
use config::error::ConfigError;
use db::error::DatabaseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IncError {
    #[error("Bcrypt error in inc layer")]
    BcryptError(#[from] BcryptError),

    #[error("Database error in inc layer")]
    DatabaseError(#[from] DatabaseError),

    #[error("Configuration error in inc layer")]
    ConfigError(#[from] ConfigError),
}
