/*
* Copyright (C) 2020  Aravinth Manivannan <realaravinth@batsense.net>
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as
* published by the Free Software Foundation, either version 3 of the
* License, or (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
//! Client wrapper for p2p communication

use actix_web::client::Client as awc;
use serde::{Deserialize, Serialize};

use damn_vuln_blockchain::asset::{Asset, ReplaceLedger, Stake};
use damn_vuln_blockchain::block::Block;
use damn_vuln_blockchain::client::*;
use damn_vuln_blockchain::config::Config;
use damn_vuln_blockchain::discovery::AddPeer;
use damn_vuln_blockchain::payload::{Peer, Status, Tx, ValidateTx};
use damn_vuln_blockchain::utils::*;

use log::debug;
//use crate::logs::SellAsset;

// NOTE these URLs are subject to change
// if tests are failing, come check the URLs
// here
/// Client wrapper for p2p communication
#[derive(Clone, Default)]
pub struct Client {
    pub client: awc,
}

/// Get stake using client
#[derive(Clone, Deserialize, Serialize)]
pub struct GetStake {
    pub block_id: usize,
    pub peer_id: String,
}

impl Client {
    /// set attack
    pub async fn set_attack(&self, peer: Peer) {
        let addr = Client::make_uri(&peer.ip, SET_ATTACK);
        self.client.post(addr).send().await.unwrap();
    }

    /// get stake for a block
    pub async fn get_stake(&self, peer: GetStake, config: &Config) -> Stake {
        use damn_vuln_blockchain::payload::GetStake as PayloadGetStake;

        let payload = PayloadGetStake {
            block_id: peer.block_id,
        };

        let peer_addr = get_peer(&config, &peer.peer_id).await;
        let addr = Client::make_uri(&peer_addr.ip, GET_STAKE);
        loop {
            if let Ok(mut val) = self
                .client
                .post(&addr)
                .header("content-type", "application/json")
                .send_json(&payload)
                .await
            {
                if let Ok(stake) = val.json().await {
                    return stake;
                }
            }
        }
    }

    fn make_uri(address: &str, path: &str) -> String {
        format!("http://{}{}", address, path)
    }

    /// gets list of peers from auditor, should be called periodically
    pub async fn peer_discovery(&self, config: &Config) {
        // gets peers from Auditor and replaces peers
        // in local Network
        let addr = Client::make_uri(&config.auditor_node, PEER_DISCOVER_ALL);
        loop {
            if let Ok(mut val) = self.client.get(&addr).send().await {
                config.debug("Peer discovery request success");
                let peers: Result<Vec<Peer>, _> = val.json().await;
                if let Ok(val) = peers {
                    println!("{:#?}", val);
                    break;
                }
            }
        }
    }

    /// send Tx request to validator
    pub async fn sell_asset(&self, config: &Config, seller_id: &str, payload: &Tx) {
        let seller = get_peer(&config, seller_id).await;
        let addr = Client::make_uri(&seller.ip, SELL_ASSET);

        loop {
            if let Ok(_) = self
                .client
                .post(&addr)
                .header("content-type", "application/json")
                .send_json(&payload)
                .await
            {
                break;
            }
        }
    }

    /// Get chain dump
    pub async fn get_chain(&self, peer: Peer) -> Vec<Block> {
        let addr = Client::make_uri(&peer.ip, GET_CHAIN);
        loop {
            if let Ok(mut val) = self.client.get(&addr).send().await {
                let blocks: Result<Vec<Block>, _> = val.json().await;
                if let Ok(blocks) = blocks {
                    println!("{:#?}", blocks);
                }
            }
        }
    }

    /// gets list of peers from auditor, should be called periodically
    pub async fn peer_dump(&self, peer: &Peer) -> Vec<Peer> {
        // gets peers from Auditor and replaces peers
        // in local Network
        let addr = Client::make_uri(&peer.ip, PEER_DISCOVER_ALL);
        loop {
            if let Ok(mut val) = self.client.get(&addr).send().await {
                //                config.debug("Peer discovery request success");
                let peers: Result<Vec<Peer>, _> = val.json().await;
                if let Ok(val) = peers {
                    println!("{:#?}", val);
                }
            }
        }
    }

    /// gets asset ledger from auditor node, should be called periodically
    pub async fn get_peer_assets(&self, peer: &Peer) {
        // gets assets from Auditor and replaces assets
        // in local AssetsLedger
        let addr = Client::make_uri(&peer.ip, GET_ALL_ASSETS);
        loop {
            if let Ok(mut val) = self.client.get(&addr).send().await {
                let peers: Result<Vec<Asset>, _> = val.json().await;
                if let Ok(val) = peers {
                    println!("{:#?}", val);
                    break;
                }
            }
        }
    }

    /// gets list of peers from auditor, should be called periodically
    pub async fn discovery(&self, target_peer: &str, auditor_ip: &str) -> Option<Peer> {
        let addr = Client::make_uri(&auditor_ip, PEER_DISCOVER_ALL);
        loop {
            if let Ok(mut val) = self.client.get(&addr).send().await {
                //                config.debug("Peer discovery request success");
                let peers: Result<Vec<Peer>, _> = val.json().await;
                if let Ok(val) = peers {
                    // NOTE: need to return target peer's Peer config
                    debug!("{:#?}", &val);
                    for peer in val {
                        if peer.id == target_peer {
                            return Some(peer);
                        }
                    }
                    return None;
                }
            }
        }
    }
}
