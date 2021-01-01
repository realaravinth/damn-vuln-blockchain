/*
* Copyright (C) 2020  Aravinth Manivannan <realaravinth@batsense.net>
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as
* published by the Free Software Foundation, either version 3 of the
* License, or (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use data_encoding::HEXUPPER;
use sha2::{Digest, Sha256};

/// helper function for generating sha256 hashes
pub fn hasher(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&content);
    let hash = hasher.finalize();
    HEXUPPER.encode(&hash)
}

/// helper function for generating random strings
/// of length = `len`
pub fn get_rand_string(len: usize) -> String {
    use std::iter;

    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(len)
        .collect()
}

/// helper function to get current timesamp
pub fn get_current_time() -> String {
    use chrono::prelude::*;
    Local::now().to_string()
}

///// helper function to get time as string since UNIX_EPOCH
//pub fn timesamp_to_string(timestamp: Timestamp) -> String {
//    unimplemented!()
//}
