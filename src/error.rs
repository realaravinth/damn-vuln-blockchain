// use std::convert::From;

use derive_more::{Display, Error};

/// Errors that can occur when interacting with the blockchain
#[derive(Debug, PartialEq, Display, Clone, Error)]
#[cfg(not(tarpaulin_include))]
pub enum ChainError {
    /// when the a genesis block is passed to get added to a chain
    /// genesis blocks are supposed to be used only during chain intiation
    /// they are invalid when they are used elsewhere
    #[display(fmt = "Block passed is a genesis block. Can't add second Genesis block")]
    GenesisBlockAdditionError,
    /// Blockchain invalid when block.hash() != block.next().prev
    /// Occours when entire blockchain's validity is checked
    #[display(fmt = "Invalid blockchain, looks like it's been tampered!")]
    InvalidBlockChain,
    /// Block inconsistent, block.hash() != chain.get_last_block*().get_hash()
    #[display(fmt = "Block can't be added, previous hash and block data don't match")]
    InconsistentBlockAdition,
}

pub type ChainResult<V> = std::result::Result<V, ChainError>;

/// Errors that can occur when interacting with the blockchain
#[derive(Debug, PartialEq, Display, Clone, Error)]
#[cfg(not(tarpaulin_include))]
pub enum PeerError {
    /// When a non-attacking peer is asked to attack
    #[display(fmt = "Peer is not configured to attack, can't carry out attack")]
    NotAttacker,
    /// When a non-auditor peer is asked to mint assets
    #[display(fmt = "Peer is not configured to mint assets, can't mint assets")]
    NotAuditor,
    /// Blockchian error
    #[display(fmt = "{}", _0)]
    ChainError(ChainError),
}

pub type PeerResult<V> = std::result::Result<V, PeerError>;
