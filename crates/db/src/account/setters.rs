/* db/src/account/query/setters.rs
 * The purpose of this module is to provide database setters for accounts.
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

use crate::account::model::AccountModel;
use crate::error::DatabaseError;
use crate::pool;
use crate::pool::DbPool;
use crate::schema::accounts;
use diesel::expression_methods::*;
use diesel::{QueryDsl, RunQueryDsl};

pub async fn set_pic_by_acc_id(
    pool: &DbPool,
    acc_id: i32,
    pic: String,
) -> Result<AccountModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        diesel::update(accounts::table.filter(accounts::id.eq(acc_id)))
            .set(accounts::pic.eq(pic))
            .get_result::<AccountModel>(conn)
    })
    .await
}

pub async fn accept_tos_by_acc_id(
    pool: &DbPool,
    acc_id: i32,
) -> Result<AccountModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        diesel::update(accounts::table.filter(accounts::id.eq(acc_id)))
            .set(accounts::accepted_tos.eq(true))
            .get_result::<AccountModel>(conn)
    })
    .await
}

pub async fn update_account(
    pool: &DbPool,
    acc_model: AccountModel,
) -> Result<AccountModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        diesel::insert_into(accounts::table)
            .values(acc_model.clone())
            .on_conflict(accounts::username)
            .do_update()
            .set(acc_model)
            .get_result::<AccountModel>(conn)
    })
    .await
}

pub async fn set_quit_status_by_acc_id(
    pool: &DbPool,
    acc_id: i32,
    quit_status: String,
) -> Result<AccountModel, DatabaseError> {
    pool::spawn_db(pool, move |conn| {
        diesel::update(accounts::table.filter(accounts::id.eq(acc_id)))
            .set((
                accounts::quit_status.eq(Some(quit_status)),
            ))
            .get_result::<AccountModel>(conn)
    })
    .await
}

// pub async fn delete_acc_by_id(pool: &DbPool, acc_id: i32) -> Result<usize, DatabaseError> {
//     pool::spawn_db(pool, move |conn| {
//         diesel::delete(accounts::table.filter(accounts::id.eq(acc_id))).execute(conn)
//     })
//     .await
// }
