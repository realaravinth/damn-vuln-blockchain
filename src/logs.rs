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

use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::block::Block;
use crate::{asset::Asset, utils::*};

#[derive(Deserialize, Display)]
#[display(fmt = "[{}]: {}", timesamp, action)]
pub struct Command {
    pub timesamp: String,
    pub action: Action,
}

impl Command {
    pub fn new(action: Action) -> Command {
        Command {
            timesamp: get_current_time(),
            action,
        }
    }
}

#[derive(Deserialize, Display)]
pub enum Action {
    /// Initialize log
    #[display(fmt = "Initializing log...")]
    InitLog,

    /// Peer connected event, pass in peer ID
    #[display(fmt = "Peer connected: {}", _0)]
    PeerConnected(Peer),

    /// Peer connected event, pass in peer ID
    #[display(fmt = "Peer enroll: ID {}", _0.ip)]
    PeerEnroll(Peer),

    /// Miniting asset event, pass in Asset ID
    #[display(fmt = "Minting asset: {}", _0)]
    MintingAsset(Asset),

    /// Distributing asset `asset ID` to peer `peer  ID`
    #[display(fmt = "Distributing asset {} to peer {}", _0, _1)]
    DistributingAssets(Asset, Peer),

    /// Transaction request event from peer `peer ID` for asset `asset ID`
    #[display(fmt = "Transaction request for asset {} from peer {}", _0, _1)]
    TransactionRequest(Asset, Peer),

    /// Stake broadcast event from peer `peer ID` with stake
    #[display(fmt = "Stake Broadcast Peer {} has stake {}", _0, _1)]
    StakeBroadcast(Peer, String),

    /// Transaction validation event. Transaction validated by peer `peer ID`
    #[display(fmt = "Transaction validation peer {}", _0)]
    TransactionValidated(Peer),

    /// Block creation event for block `block ID`
    #[display(fmt = "Block creation Block ID {}" _0)]
    BlockCreation(Block),

    /// Transaction broadcasting event by peer `peer ID` to peer `peer ID`
    /// after when a block is created
    #[display(fmt = "")]
    TransactionBroadcasting,
}

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

/// Sell asset payload
#[derive(Deserialize, Serialize)]
pub struct SellAsset {
    /// asset ID
    pub asset_id: String,
    /// use stake for transaction?
    pub use_stake: bool,
}
