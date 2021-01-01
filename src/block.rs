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

use crate::asset::Asset;

/// Builder struct to create blocks
/// ```rust
/// use damn_vuln_blockchain::{ asset::AssetLedger, block::{BlockBuilder, Block}};
///
/// fn main() {
///        let prev = Block::genesis(); // genesis block
///        let mut assets = AssetLedger::generate(); // generate some assets
///
///        let asset = assets.assets.pop().unwrap();
///
///        let block = BlockBuilder::default()
///            .set_tx("Me")
///            .set_rx("You")
///            .set_prev(&prev)
///            .set_asset_id(&asset)
///            .build();
///        assert_eq!(block.get_tx(), "Me");
///        assert_eq!(block.get_rx(), "You");
/// }

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct BlockBuilder {
    prev: String,
    tx: String,
    rx: String,
    asset_id: String,
}

impl BlockBuilder {
    /// set previous block's hash
    pub fn set_prev(&mut self, prev: &Block) -> &mut Self {
        self.prev = prev.get_hash().into();
        self
    }

    /// set receiver's address
    pub fn set_rx(&mut self, rx: &str) -> &mut Self {
        self.rx = rx.into();
        self
    }

    /// set sender's address
    pub fn set_tx(&mut self, tx: &str) -> &mut Self {
        self.tx = tx.into();
        self
    }

    /// set assset ID
    pub fn set_asset_id(&mut self, assset: &Asset) -> &mut Self {
        self.asset_id = assset.get_hash().into();
        self
    }

    /// Build block, this method must be called at the very end
    pub fn build(&mut self) -> Block {
        use crate::utils::hasher;
        if self.prev.is_empty()
            || self.rx.is_empty()
            || self.tx.is_empty()
            || self.asset_id.is_empty()
        {
            panic!("Can't create block, one or more fields are empty");
        } else {
            let hash = hasher(&format!("{}{}{}", self.prev, self.rx, self.tx));
            Block {
                prev: Some(self.prev.to_owned()),
                tx: Some(self.tx.to_owned()),
                rx: Some(self.rx.to_owned()),
                hash,
            }
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Block {
    prev: Option<String>,
    hash: String,
    tx: Option<String>,
    rx: Option<String>,
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Previous Block: {}\nHash: {}\nSender: {}\nReceiver: {}\n",
            self.get_prev(),
            self.get_hash(),
            self.get_rx(),
            self.get_tx()
        )
    }
}

impl Block {
    /// First block of a blockchain
    pub fn genesis() -> Self {
        use crate::utils::{get_rand_string, hasher};

        let hash = hasher(&get_rand_string(10));
        Block {
            prev: None,
            tx: None,
            rx: None,
            hash,
        }
    }

    pub fn get_prev(&self) -> &str {
        match &self.prev {
            Some(val) => val,
            None => "Genesis block",
        }
    }

    pub fn get_hash(&self) -> &str {
        &self.hash
    }

    pub fn get_rx(&self) -> &str {
        match &self.rx {
            Some(val) => val,
            None => "Genesis block",
        }
    }

    pub fn get_tx(&self) -> &str {
        match &self.tx {
            Some(val) => val,
            None => "Genesis block",
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn block_works() {
        use crate::asset::AssetLedger;

        let prev = Block::genesis();
        let mut assets = AssetLedger::generate();
        let asset = assets.assets.pop().unwrap();

        let block = BlockBuilder::default()
            .set_tx("Me")
            .set_rx("You")
            .set_prev(&prev)
            .set_asset_id(&asset)
            .build();
        assert_eq!(block.get_tx(), "Me");
        assert_eq!(block.get_rx(), "You");
    }

    #[test]
    #[should_panic]
    fn block_panic_works() {
        let prev = Block::genesis();

        let _ = BlockBuilder::default()
            .set_rx("You")
            .set_tx("Me")
            .set_prev(&prev)
            .build();
    }

    #[test]
    #[should_panic]
    fn block_panic2_works() {
        let prev = Block::genesis();

        let _ = BlockBuilder::default().set_prev(&prev).build();
    }

    #[test]
    #[should_panic]
    fn block_panic3_works() {
        let _ = BlockBuilder::default().build();
    }
}
