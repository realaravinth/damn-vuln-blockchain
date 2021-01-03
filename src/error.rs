// use std::convert::From;

use derive_more::{Display, Error};

/// Errors that can occur when interacting with the blockchain
#[derive(Debug, PartialEq, Display, Clone, Error)]
#[cfg(not(tarpaulin_include))]
pub enum ChainError {
    /// when the a genesis block is passed to get added toa chain
    /// genesis blocks are supposed to be used only during chain intiation
    /// they are invalid when they are used elsewhere
    #[display(fmt = "Block passed is a genesis block. Can't add second Genesis block")]
    GenesisBlockAdditionError,
}

pub type ChainResult<V> = std::result::Result<V, ChainError>;
