/* config/src/error.rs
 * The purpose of this module is to provide errors related to configuration.
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

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid integer value for key in config layer: {0}")]
    InvalidInt(String),

    #[error("Invalid float value for key in config layer: {0}")]
    InvalidFloat(String),

    #[error("Invalid boolean value for key in config layer: {0}")]
    InvalidBool(String),

    #[error("Invalid string value for key in config layer: {0}")]
    InvalidString(String),

    #[error("Integer conversion error in config layer")]
    IntConversion(#[from] std::num::TryFromIntError),

    #[error("Config backend error in config layer")]
    Source(#[from] cfg::ConfigError),
}
