/* test-suite/src/tests/login_tests.rs
 * The purpose of this module is to test the quit app's login flow.
 *
 * Copyright (C) 2026  https://github.com/brandongrahamcobb/Quit.git
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::time::SystemTime;

use db::account::model::{AccountModel, NewAccountModel};
use db::account::{getters, setters};
use db::error::DatabaseError;
use db::pool::DbPool;

const ADMIN_USERNAME: &str = "admin";
const ADMIN_PASSWORD: &str = "admin";
const ADMIN_PIC: &str = "123456";
const ADMIN_BANNED: bool = false;
const ADMIN_ACCEPTED_TOS: bool = true;
const ADMIN_GENDER: &str = "Male";
const USER_USERNAME: &str = "user";
const USER_PASSWORD: &str = "user";
const USER_PIC: &str = "123456";
const USER_QUIT_STATUS: &str = "I want to quit smoking because its bad for my health.";
const USER_BANNED: bool = false;
const USER_ACCEPTED_TOS: bool = true;
const USER_GENDER: &str = "Male";


// fn test_login() {


async fn create_user_account(pool: &DbPool) -> Result<AccountModel, DatabaseError> {
    let new_user: NewAccountModel = NewAccountModel {
        username: USER_USERNAME,
        password: USER_PASSWORD,
        pic: USER_PIC,
        gender: USER_GENDER,
        accepted_tos: USER_ACCEPTED_TOS,
        banned: USER_BANNED,
        admin: false,
        quit_status: USER_QUIT_STATUS
    };
    let user: AccountModel = setters::insert_account(pool, new_user).await?;
    Ok(user)
}


async fn create_admin_account(pool: &DbPool) -> Result<AccountModel, DatabaseError> {
    let new_admin: NewAccountModel = NewAccountModel {
        username: ADMIN_USERNAME,
        password: ADMIN_PASSWORD,
        pic: ADMIN_PIC,
        gender: ADMIN_GENDER,
        accepted_tos: ADMIN_ACCEPTED_TOS,
        banned: ADMIN_BANNED,
        admin: true,
        quit_status: None
    };
    let admin: AccountModel = setters::insert_account(pool, new_admin).await?;
    Ok(admin)
}

async fn test_account_getters(pool: &DbPool, admin_acc: AccountModel, user_acc: AccountModel) -> Result<(), DatabaseError> {
    let admin: AccountModel = getters::get_acc_model_by_id(pool, admin_acc.id).await?;
    assert!(admin.username, ADMIN_USERNAME);
    assert!(admin.password, ADMIN_PASSWORD);
    assert!(admin.pic, ADMIN_PIC);
    assert!(admin.gender, ADMIN_GENDER);
    assert!(admin.accepted_tos, ADMIN_ACCEPTED_TOS);
    assert!(admin.banned, ADMIN_BANNED);
    assert!(admin.admin, true);
    assert!(admin.quit_status, None);

    let user: AccountModel = getters::get_acc_model_by_id(pool, user_acc.id).await?;
    assert!(user.username, USER_USERNAME);
    assert!(user.password, USER_PASSWORD);
    assert!(user.pic, USER_PIC);
    assert!(user.gender, USER_GENDER);
    assert!(user.accepted_tos, USER_ACCEPTED_TOS);
    assert!(user.banned, USER_BANNED);
    assert!(user.admin, false);
    assert!(user.quit_status, USER_QUIT_STATUS);

    let admin: AccountModel = getters::get_acc_model_by_username(pool, admin_acc.username).await?;
    assert!(admin.username, ADMIN_USERNAME);
    assert!(admin.password, ADMIN_PASSWORD);
    assert!(admin.pic, ADMIN_PIC);
    assert!(admin.gender, ADMIN_GENDER);
    assert!(admin.accepted_tos, ADMIN_ACCEPTED_TOS);
    assert!(admin.banned, ADMIN_BANNED);
    assert!(admin.admin, true);
    assert!(admin.quit_status, None);

    let user: AccountModel = getters::get_acc_model_by_username(pool, user_acc.username).await?;
    assert!(user.username, USER_USERNAME);
    assert!(user.password, USER_PASSWORD);
    assert!(user.pic, USER_PIC);
    assert!(user.gender, USER_GENDER);
    assert!(user.accepted_tos, USER_ACCEPTED_TOS);
    assert!(user.banned, USER_BANNED);
    assert!(user.admin, false);
    assert!(user.quit_status, USER_QUIT_STATUS);

    Ok(())
}




