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

//! Error datatypes
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

/// [Result] datatype for Chain interactions
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

/// [Result] datatype for peer interactions
pub type PeerResult<V> = std::result::Result<V, PeerError>;
