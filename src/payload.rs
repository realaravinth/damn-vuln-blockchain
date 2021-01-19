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
//! Payload datatype that `dwb` uses
use derive_builder::Builder;
use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::asset::Asset;
use crate::block::Block;

/// Represents a peer
#[derive(Deserialize, Display, Serialize, Clone, Debug, Default)]
#[display(fmt = "{}", id)]
pub struct Peer {
    /// some random ID
    pub id: String,
    /// IP must include the port as well
    pub ip: String,
    //pub balance: Option<u64>,
}

#[derive(Deserialize, Display, Serialize, Clone, Debug, Default)]
#[display(fmt = "from {} to {}", tx, rx)]
pub struct Gossip {
    pub tx: Peer,
    pub rx: Peer,
}

/// Transaction payload
#[derive(Deserialize, Builder, Clone, Serialize)]
pub struct Tx {
    /// asset ID
    pub asset_id: String,
    /// buyer peer ID
    pub buyer_peer_id: String,
}

/// Get stake payload
#[derive(Deserialize, Serialize)]
pub struct GetStake {
    pub block_id: usize,
}

/// Transaction payload
#[derive(Deserialize, Builder, Serialize)]
pub struct ValidateTx {
    /// Transaction request
    pub tx: Tx,
    /// seller peer ID
    pub seller_peer_id: String,
}

/// Transaction payload
#[derive(Deserialize, Builder, Serialize)]
pub struct Status {
    pub peer_id: String,
    pub asset: Vec<Asset>,
    pub tampered_assets: Option<Vec<Asset>>,
    pub chain: Vec<Block>,
    pub tampered_chain: Option<Vec<Block>>,
}
