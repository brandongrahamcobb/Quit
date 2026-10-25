/* inc/src/account.rs
 * The purpose of this module is to provide helper methods for accounts.
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

use base::account::ValidAccountCode;
use bcrypt::verify;
use db::account::model::AccountModel;

use crate::error::IncError;

pub fn check_pic(acc_pic: Option<String>, pic: String) -> bool {
    acc_pic == Some(pic)
}

pub fn authenticate(acc_pw: String, pw: String) -> Result<bool, IncError> {
    verify(&pw, &acc_pw).map_err(IncError::BcryptError)
}

pub fn get_status_code_by_account(acc_model: &AccountModel) -> ValidAccountCode {
    if acc_model.banned {
        return ValidAccountCode::Banned;
    }
    if !acc_model.accepted_tos {
        return ValidAccountCode::PendingTOS;
    }
    ValidAccountCode::Success
}
