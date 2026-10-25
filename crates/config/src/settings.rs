/* config/src/settings.rs
 * The purpose of this module is to access to values in .env.
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
use crate::error::ConfigError;
use cfg::Config;

pub fn get_settings() -> Result<Config, ConfigError> {
    let settings = Config::builder()
        .add_source(cfg::Environment::default())
        .build()
        .unwrap();
    Ok(settings)
}

pub fn get_release_mode() -> Result<bool, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "release_mode";
    let mode = settings
        .get_bool(key)
        .map_err(|_| ConfigError::InvalidString(key.to_string()))?;
    Ok(mode)
}

pub fn get_version() -> Result<i16, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "version";
    let version = settings
        .get_int(key)
        .map_err(|_| ConfigError::InvalidInt(key.to_string()))?;
    Ok(version as i16)
}

pub fn get_db_url() -> Result<String, ConfigError> {
    let settings = get_settings()?;
    let db_key: &str = "postgres_database";
    let ip_key: &str = "postgres_host";
    let port_key: &str = "postgres_port";
    let user_key: &str = "postgres_user";
    let pw_key: &str = "postgres_password";
    let db = settings
        .get_string(db_key)
        .map_err(|_| ConfigError::InvalidString(db_key.to_string()))?;
    let ip = settings
        .get_string(ip_key)
        .map_err(|_| ConfigError::InvalidString(ip_key.to_string()))?;
    let port = settings
        .get_int(port_key)
        .map_err(|_| ConfigError::InvalidInt(port_key.to_string()))?;
    let user = settings
        .get_string(user_key)
        .map_err(|_| ConfigError::InvalidString(user_key.to_string()))?;
    let pw = settings
        .get_string(pw_key)
        .map_err(|_| ConfigError::InvalidString(pw_key.to_string()))?;
    Ok(format!("postgres://{}:{}@{}:{}/{}", user, pw, ip, port, db))
}

pub fn get_pic_required() -> Result<bool, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "pic_required";
    let pic_req = settings
        .get_bool(key)
        .map_err(|_| ConfigError::InvalidBool(key.to_string()))?;
    Ok(pic_req)
}

pub fn get_gender_required() -> Result<bool, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "gender_required";
    let gender_req = settings
        .get_bool(key)
        .map_err(|_| ConfigError::InvalidBool(key.to_string()))?;
    Ok(gender_req)
}
