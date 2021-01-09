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

use actix::prelude::*;
use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::asset::Asset;
use crate::error::*;

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
/// Block. `tx`, `prev`, validator and `rx` are `Option<_>` to accomodate
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

    /// checks if the block is a genesis block
    pub fn is_genesis(&self) -> bool {
        if self.prev.is_none() || self.tx.is_none() || self.tx.is_none() || self.rx.is_none() {
            return true;
        }
        false
    }

    // creates genesis block
    fn genesis() -> Block {
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

    /// computes the hash of a block, uses the same logic
    /// for genesis blocks, it simply returns the hash stored
    /// in the block as genesis() computes hash over random
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Chain {
    name: String,
    blocks: Vec<Block>,
}

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
    pub fn add_block(&mut self, block: Block) -> ChainResult<()> {
        if block.is_genesis() {
            return Err(ChainError::GenesisBlockAdditionError);
        // unwrap() is fine below because `block` is not genesis
        } else if block.get_prev().unwrap() != self.get_last_block().get_hash() {
            return Err(ChainError::InconsistentBlockAdition);
        } else {
            self.blocks.push(block);
        }
        Ok(())
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
#[derive(Message)]
#[rtype(result = "ChainResult<()>")]
pub struct AddBlock(Block);

/// Get last block
#[derive(Message)]
#[rtype(result = "Block")]
pub struct GetLastBlock;

/// Replace Chain
#[derive(Message)]
#[rtype(result = "ChainResult<()>")]
pub struct ReplaceChain(Vec<Block>);

impl Handler<AddBlock> for Chain {
    type Result = MessageResult<AddBlock>;

    fn handle(&mut self, msg: AddBlock, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.add_block(msg.0))
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn block_works() {
        use crate::asset::AssetLedger;

        let prev = Block::genesis();
        assert_eq!(prev.is_genesis(), true, "Genesis block identified");

        assert_eq!(prev.hash(), prev.hash, "Genesis block hash works");
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
        assert_eq!(block.hash(), block.hash, "non-genesis block hash works");
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

    #[test]
    fn chain_works() {
        use crate::asset::AssetLedger;

        let mut chain = Chain::new("test chain");

        let prev = chain.get_last_block();

        let mut assets = AssetLedger::generate();
        let asset = assets.assets.pop().unwrap();

        let block = BlockBuilder::default()
            .set_tx("Me")
            .set_rx("You")
            .set_prev(&prev)
            .set_asset_id(&asset)
            .build();

        assert_eq!(
            chain.add_block(Block::genesis()),
            Err(ChainError::GenesisBlockAdditionError),
            "Genesis Block addition prevented"
        );

        chain.add_block(block.clone()).unwrap();
        assert_eq!(
            chain.get_last_block().hash(),
            block.hash(),
            "add_block works"
        );

        assert_eq!(
            chain.add_block(block),
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

        assert_eq!(
            chain_addr.send(AddBlock(Block::genesis())).await.unwrap(),
            Err(ChainError::GenesisBlockAdditionError),
            "Genesis Block addition prevented"
        );

        chain_addr
            .send(AddBlock(block.clone()))
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            chain_addr.send(GetLastBlock).await.unwrap().hash(),
            block.hash(),
            "add_block works"
        );

        assert_eq!(
            chain_addr.send(AddBlock(block.clone())).await.unwrap(),
            Err(ChainError::InconsistentBlockAdition),
            "Chain Invalid Prevention works"
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

        // craete invalid block chain
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

        // add new block
        chain_addr
            .send(AddBlock(new_block.clone()))
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
