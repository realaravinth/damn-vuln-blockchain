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

use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Asset {
    name: String,
    value: usize,
    hash: String,
    owner: Option<String>,
}

impl Display for Asset {
    #[cfg(not(tarpaulin_include))]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let owner = if let Some(val) = self.get_owner() {
            val
        } else {
            "Fresh out of mint"
        };
        write!(
            f,
            "Hash:{}\nName: {}\nValue: {}\nOwner: {}",
            self.get_hash(),
            self.get_name(),
            self.get_value(),
            owner
        )
    }
}

impl Asset {
    /// create new asset
    pub fn new(name: &str, value: usize) -> Self {
        use super::utils::{get_rand_string, hasher};

        let hash = hasher(&format!("{}-{}{}", get_rand_string(10), &name, &value));
        Asset {
            name: name.into(),
            value,
            owner: None,
            hash,
        }
    }

    /// get name of the asset
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// get hash of the asset
    pub fn get_hash(&self) -> &str {
        &self.hash
    }

    /// get value/price of the asset
    pub fn get_value(&self) -> usize {
        self.value
    }

    /// set owner of the asset
    pub fn set_owner(&mut self, owner: &str) {
        self.owner = Some(owner.into());
    }

    /// get owner of the asset
    pub fn get_owner(&self) -> &Option<String> {
        &self.owner
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AssetLedger {
    pub assets: Vec<Asset>,
}

impl AssetLedger {
    pub fn generate() -> AssetLedger {
        let mut ledger = AssetLedger { assets: Vec::new() };

        ledger.assets.push(Asset::new("les Escaldes", 100));
        ledger.assets.push(Asset::new("Andorra la Vella", 100));
        ledger.assets.push(Asset::new("Umm al Qaywayn", 100));
        ledger.assets.push(Asset::new("Ras al-Khaimah", 100));
        ledger.assets.push(Asset::new("Khawr Fakkān", 100));
        ledger.assets.push(Asset::new("Dubai", 100));
        ledger.assets.push(Asset::new("Dibba Al-Fujairah", 100));
        ledger.assets.push(Asset::new("Dibba Al-Hisn", 100));
        ledger.assets.push(Asset::new("Sharjah", 100));
        ledger.assets.push(Asset::new("Ar Ruways", 100));
        ledger.assets.push(Asset::new("Al Fujayrah", 100));
        ledger.assets.push(Asset::new("Al Ain", 100));
        ledger.assets.push(Asset::new("Ajman", 100));
        ledger.assets.push(Asset::new("Adh Dhayd", 100));
        ledger.assets.push(Asset::new("Abu Dhabi", 100));
        ledger
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn mint_asset_works() {
        let mut asset = Asset::new("Delhi", 100);
        assert_eq!(asset.get_name(), "Delhi");
        assert_eq!(asset.get_owner(), &None);
        assert_eq!(asset.get_value(), 100);

        let new_owner = "Me".to_string();
        asset.set_owner(&new_owner);
        assert_eq!(asset.get_owner(), &Some(new_owner));
    }
}
