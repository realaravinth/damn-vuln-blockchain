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

//! Ledger data-structure for the blockchain
//!
//! ## Create a new blockchain
//! ```rust
//! use damn_vuln_blockchain::{ asset::AssetLedger, block::{BlockBuilder, Block}, chain::Chain};
//!
//! fn main() {
//!        let chain = Chain::new("My chain"); // crate cahin
//!   }
//! ```
//!
//! The easiest way to interact with the ledger is via the [Chain] actor.
//!
//! # [Chain] supports the followings messages:
//! - [AddBlock]: adds a [Block] to the blockchain
//! - [GetLastBlock]: get's the latest [Block] in the blockchain
//! - [DumpLedger]: dumps the entire ledger
//! - [ReplaceChain]: replaces a [Vec<Block>] inside the [Chain] data-structure, useful
//! when synchronising ledgers

use actix::prelude::*;
use serde::{Deserialize, Serialize};

use crate::block::Block;
use crate::error::*;

/// Ledger data-structure for the blockchain
///
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Chain {
    name: String,
    blocks: Vec<Block>,
}

/// # [Chain] supports the followings messages:
/// - [AddBlock]: adds a [Block] to the blockchain
/// - [GetLastBlock]: get's the latest [Block] in the blockchain
/// - [DumpLedger]: dumps the entire ledger
/// - [ReplaceChain]: replaces a [Vec<Block>] inside the [Chain] data-structure, useful
/// when synchronising ledgers

impl Chain {
    /// create new blockchain
    pub fn new(name: &str) -> Chain {
        let genesis = Block::genesis();
        let blocks = vec![genesis];
        Chain {
            name: name.into(),
            blocks,
        }
    }

    /// get the last block in the chain
    pub fn get_last_block(&self) -> &Block {
        // unwrap is okay here because chain initiation guarentees
        // block creation. Sot self.blocks.last() will always
        // return Some(Block)
        self.blocks.last().unwrap()
    }

    /// add a block to the chain
    /// ChainError::GenesisBlockAdditionError error is returned when
    /// a genesis block is passed. Genesis blocks are only valid when
    /// a chain is created.
    pub fn add_block(&mut self, mut block: Block, network_size: usize) -> ChainResult<usize> {
        if block.is_genesis() {
            return Err(ChainError::GenesisBlockAdditionError);
        // unwrap() is fine below because `block` is not genesis
        } else if block.get_prev().unwrap() != self.get_last_block().get_hash() {
            return Err(ChainError::InconsistentBlockAdition);
        } else {
            // unwrap is okay here because [Block::genesis()] starts with
            // serial_no = 1 and every other block that gets added to the chain
            // will have its serial number set
            let last_serial_no = self.get_last_block().get_serial_no().unwrap();
            let serial_no = if last_serial_no == 0 {
                network_size + 1
            } else {
                last_serial_no + 1
            };

            block.set_serial_no(serial_no);
            self.blocks.push(block);
            return Ok(serial_no);
        }
    }

    /// checks if a blockchain is valid by comparing the hash of the previous
    /// element with the block.prev of the next element in the blockchain
    pub fn is_valid(chain: &Vec<Block>) -> ChainResult<()> {
        let mut iter = chain.iter().peekable();
        loop {
            if let Some(val) = iter.next() {
                if let Some(next) = iter.peek() {
                    if &val.hash() != next.get_prev().unwrap() {
                        //unwrap is okay
                        // here as we'll only be passing non-genesis blocks
                        return Err(ChainError::InvalidBlockChain);
                    }
                }
            } else {
                break;
            }
        }
        Ok(())
    }

    pub fn replace_chain(&mut self, chain: Vec<Block>) -> ChainResult<()> {
        Chain::is_valid(&chain)?;
        self.blocks = chain;
        Ok(())
    }
}

impl Actor for Chain {
    type Context = Context<Self>;
}

/// Add Block
/// send block and network_size
/// network_size is required becuase when InitNetwork is called
/// it sets an offset for Block.serial_no = network_size
#[derive(Message)]
#[rtype(result = "ChainResult<usize>")]
pub struct AddBlock(pub Block, pub usize);

/// Get last block
#[derive(Message)]
#[rtype(result = "Block")]
pub struct GetLastBlock;

/// Replace Chain
#[derive(Message)]
#[rtype(result = "ChainResult<()>")]
pub struct ReplaceChain(pub Vec<Block>);

/// Dumps entire ledger
/// Useful when forking:
/// send `DumpLedger` and send output with `ReplaceChain`
#[derive(Message)]
#[rtype(result = "Vec<Block>")]
pub struct DumpLedger;

impl Handler<AddBlock> for Chain {
    type Result = MessageResult<AddBlock>;

    fn handle(&mut self, msg: AddBlock, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.add_block(msg.0, msg.1))
    }
}

impl Handler<GetLastBlock> for Chain {
    type Result = MessageResult<GetLastBlock>;

    fn handle(&mut self, _msg: GetLastBlock, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.get_last_block().to_owned())
    }
}

impl Handler<ReplaceChain> for Chain {
    type Result = MessageResult<ReplaceChain>;

    fn handle(&mut self, msg: ReplaceChain, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.replace_chain(msg.0))
    }
}

impl Handler<DumpLedger> for Chain {
    type Result = MessageResult<DumpLedger>;

    fn handle(&mut self, _msg: DumpLedger, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.blocks.clone())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::block::*;

    #[test]
    fn chain_works() {
        use crate::asset::AssetLedger;

        let mut chain = Chain::new("test chain");
        assert_eq!(
            chain.get_last_block().get_serial_no().unwrap(),
            0,
            "genesis serial number properly set"
        );

        let prev = chain.get_last_block();

        let mut assets = AssetLedger::generate();
        let asset = assets.assets.pop().unwrap();

        let block = BlockBuilder::default()
            .set_tx("Me")
            .set_rx("You")
            .set_prev(&prev)
            .set_asset_id(&asset)
            .build();
        let network_size = 3;

        assert_eq!(
            chain.add_block(Block::genesis(), network_size),
            Err(ChainError::GenesisBlockAdditionError),
            "Genesis Block addition prevented"
        );

        chain.add_block(block.clone(), network_size).unwrap();
        assert_eq!(
            chain.get_last_block().hash(),
            block.hash(),
            "add_block works"
        );

        assert_eq!(
            chain.get_last_block().get_serial_no().unwrap(),
            4,
            "serial number properly set"
        );

        assert_eq!(
            chain.add_block(block, network_size),
            Err(ChainError::InconsistentBlockAdition),
            "Chain Invalid Prevention works"
        );
    }

    #[actix_rt::test]
    async fn chain_actor_works() {
        use crate::asset::AssetLedger;

        let chain_addr = Chain::new("test chain").start();

        let prev = chain_addr.send(GetLastBlock).await.unwrap();

        let mut assets = AssetLedger::generate();
        let asset = assets.assets.pop().unwrap();

        let block = BlockBuilder::default()
            .set_tx("Me")
            .set_rx("You")
            .set_prev(&prev)
            .set_asset_id(&asset)
            .build();

        let network_size = 3;

        // checks if genesis block can be appended to a blockchian
        assert_eq!(
            chain_addr
                .send(AddBlock(Block::genesis(), network_size))
                .await
                .unwrap(),
            Err(ChainError::GenesisBlockAdditionError),
            "Genesis Block addition prevented"
        );

        // checks if valid blocks can be added to blockchian
        chain_addr
            .send(AddBlock(block.clone(), network_size))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            chain_addr.send(GetLastBlock).await.unwrap().hash(),
            block.hash(),
            "add_block works"
        );

        // checks if invalid block, where block.get_prev() != chain.get_last_block().get_hash()
        // can be added to chain
        assert_eq!(
            chain_addr
                .send(AddBlock(block.clone(), network_size))
                .await
                .unwrap(),
            Err(ChainError::InconsistentBlockAdition),
            "Chain Invalid Prevention works"
        );
        let dump = chain_addr.send(DumpLedger).await.unwrap().pop().unwrap();

        // checks if dump works by popping the last element of the dump and getting
        // its hash and comparing it with the chain's last element's hash
        assert_eq!(
            chain_addr.send(GetLastBlock).await.unwrap().hash(),
            dump.get_hash(),
            "Dump works"
        );
    }

    #[actix_rt::test]
    async fn chain_replace_works() {
        use crate::asset::AssetLedger;

        // create parallel_chain to get genesis hash
        let parallel_chain = Chain::new("test chain");

        let prev = parallel_chain.get_last_block();

        let mut assets = AssetLedger::generate();
        let asset = assets.assets.pop().unwrap();

        let block = BlockBuilder::default()
            .set_tx("Me")
            .set_rx("You")
            .set_prev(&prev)
            .set_asset_id(&asset)
            .build();

        //        let main_last_block_hash = chain_addr.send(GetLastBlock).await.unwrap().get_hash();
        // get parallel_chain's hash
        let parallel_last_block_hash = block.get_hash();

        // create invalid block chain
        let chain_invalid = vec![block.clone(), block.clone()];
        assert_eq!(
            Chain::is_valid(&chain_invalid),
            Err(ChainError::InvalidBlockChain),
            "Invalid Blockchain test"
        );

        // create valid blockchain
        let parallel_chain_valid = vec![prev.clone(), block.clone()];

        // create chain that needs to be replaced
        let chain_addr = Chain::new("test chain").start();

        // get previous block to add new block
        let prev = chain_addr.send(GetLastBlock).await.unwrap();

        let mut assets = AssetLedger::generate();
        let asset = assets.assets.pop().unwrap();

        let new_block = BlockBuilder::default()
            .set_tx("Me")
            .set_rx("You")
            .set_prev(&prev)
            .set_asset_id(&asset)
            .build();

        let network_size = 3;

        // add new block
        chain_addr
            .send(AddBlock(new_block.clone(), network_size))
            .await
            .unwrap()
            .unwrap();

        // attempt replace
        chain_addr
            .send(ReplaceChain(parallel_chain_valid))
            .await
            .unwrap()
            .unwrap();

        // check if it's been replaced
        assert_eq!(
            chain_addr.send(GetLastBlock).await.unwrap().get_hash(),
            parallel_last_block_hash,
            "Chain Replaced"
        );

        // attempt replace with invalid blockchain
        assert_eq!(
            chain_addr.send(ReplaceChain(chain_invalid)).await.unwrap(),
            Err(ChainError::InvalidBlockChain),
            "Invalild blockchain replace test"
        );
    }
}
