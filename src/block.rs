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
//! the smallest unit/data-structure that can go into the blockchain
//! ## Create a block
//! ```rust
//! use damn_vuln_blockchain::{ asset::AssetLedger, block::{BlockBuilder, Block}, chain::Chain};
//!
//!
//! fn main() {
//!        let chain = Chain::new("My chain"); // create blockchain
//!        let mut assets = AssetLedger::generate(); // generate some assets
//!
//!        let asset = assets.assets.pop().unwrap();
//!
//!        // get the last block of a chain
//!        let prev = chain.get_last_block();
//!
//!        let block = BlockBuilder::default()
//!            .set_tx("Me")
//!            .set_rx("You")
//!            .set_prev(&prev)
//!            .set_asset_id(&asset)
//!            .set_validator("Me")
//!            .build();
//!
//!        assert!(!block.is_genesis());
//!        assert_eq!(block.get_tx().unwrap(), "Me");
//!        assert_eq!(block.get_rx().unwrap(), "You");
//! }
//! ```

use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::asset::Asset;

/// Builder struct for [Block]

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct BlockBuilder {
    /// previous block's hash
    prev: String,
    /// sender's peer ID
    tx: String,
    /// receiver's peer ID
    rx: String,
    /// asset ID
    asset_id: String,
    /// validator's ID
    validator: String,
}

impl BlockBuilder {
    /// set previous block's hash
    pub fn set_prev(&mut self, prev: &Block) -> &mut Self {
        self.prev = prev.get_hash().into();
        self
    }

    /// set receiver's ID
    pub fn set_rx(&mut self, rx: &str) -> &mut Self {
        self.rx = rx.into();
        self
    }

    /// set sender's ID
    pub fn set_tx(&mut self, tx: &str) -> &mut Self {
        self.tx = tx.into();
        self
    }

    /// set validator's ID
    pub fn set_validator(&mut self, validator: &str) -> &mut Self {
        self.validator = validator.into();
        self
    }

    /// set assset ID
    pub fn set_asset_id(&mut self, assset: &Asset) -> &mut Self {
        self.asset_id = assset.get_hash().into();
        self
    }

    fn hash(&self) -> String {
        use crate::utils::*;
        hasher(&format!("{}{}{}", self.prev, self.rx, self.tx))
    }

    /// Build block, this method must be called at the very end
    pub fn build(&mut self) -> Block {
        use crate::utils::*;
        if self.prev.is_empty()
            || self.rx.is_empty()
            || self.tx.is_empty()
            || self.asset_id.is_empty()
        {
            panic!("Can't create block, one or more fields are empty");
        } else {
            let hash = self.hash();
            Block {
                prev: Some(self.prev.to_owned()),
                tx: Some(self.tx.to_owned()),
                rx: Some(self.rx.to_owned()),
                hash,
                validator: Some(self.validator.to_owned()),
                timesamp: get_current_time(),
            }
        }
    }
}

#[derive(Display, Deserialize, Serialize, Clone, Debug, Default)]
#[display(fmt = "{}", hash)]
/// Smallest data-sctructure that can go into [Chain](crate::chain::Chain).
///
/// NOTE: `tx`, `prev`, validator and `rx` are `Option<_>` to accomodate
/// genesis block. Blockchain implementors must check for the
/// existence of genesis block before appending the block to
/// the ledger
pub struct Block {
    prev: Option<String>,
    hash: String,
    tx: Option<String>,
    rx: Option<String>,
    timesamp: String,
    validator: Option<String>,
}

impl Block {
    /// Get block info as string
    #[cfg(not(tarpaulin_include))]
    pub fn to_string(&self) -> String {
        if self.is_genesis() {
            format!("Genesis block \nHash: {}", self.get_hash())
        } else {
            format!(
                "Previous Block: {}\nHash: {}\n Validator: {}\nSender: {}\nReceiver: {}\n",
                &self.get_prev().as_ref().unwrap(),
                &self.get_hash(),
                &self.get_validator().as_ref().unwrap(),
                &self.get_rx().as_ref().unwrap(),
                &self.get_tx().as_ref().unwrap()
            )
        }
    }

    /// creates genesis block
    pub fn genesis() -> Block {
        use crate::utils::*;

        let hash = hasher(&get_rand_string(10));
        Block {
            prev: None,
            tx: None,
            rx: None,
            hash,
            timesamp: get_current_time(),
            validator: None,
        }
    }

    /// checks if the block is a genesis block
    pub fn is_genesis(&self) -> bool {
        if self.prev.is_none() || self.tx.is_none() || self.tx.is_none() || self.rx.is_none() {
            return true;
        }
        false
    }

    /// computes the hash of a block, uses the same logic
    /// for genesis blocks, it simply returns the hash stored
    /// in the block as [genesis()](Block::genesis) computes hash over random
    /// strings
    pub fn hash(&self) -> String {
        use crate::utils::*;
        if self.is_genesis() {
            return self.get_hash().into();
        } else {
            hasher(&format!(
                "{}{}{}",
                self.prev.as_ref().unwrap(),
                self.rx.as_ref().unwrap(),
                self.tx.as_ref().unwrap()
            ))
        }
    }

    /// get hash of previous block
    pub fn get_prev(&self) -> Option<&String> {
        self.prev.as_ref()
    }

    /// get hash of block
    pub fn get_hash(&self) -> &str {
        &self.hash
    }

    /// get receiver involved in the transaction that lead tot
    /// the creation of this block
    pub fn get_rx(&self) -> Option<&String> {
        self.rx.as_ref()
    }

    /// get validator involved in the creation of this block
    pub fn get_validator(&self) -> Option<&String> {
        self.validator.as_ref()
    }

    /// get sender involved in the transaction that lead tot
    /// the creation of this block
    pub fn get_tx(&self) -> Option<&String> {
        self.tx.as_ref()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn block_works() {
        use crate::asset::AssetLedger;

        let prev = Block::genesis();
        assert_eq!(prev.is_genesis(), true, "Genesis block identified");

        assert_eq!(prev.hash(), prev.get_hash(), "Genesis block hash works");
        let mut assets = AssetLedger::generate();
        let asset = assets.assets.pop().unwrap();

        let block = BlockBuilder::default()
            .set_tx("Me")
            .set_rx("You")
            .set_prev(&prev)
            .set_asset_id(&asset)
            .build();

        assert_eq!(block.is_genesis(), false, "non-genesis block identified");
        assert_eq!(block.get_tx().unwrap(), "Me");
        assert_eq!(block.get_rx().unwrap(), "You");
        assert_eq!(
            block.hash(),
            block.get_hash(),
            "non-genesis block hash works"
        );
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
