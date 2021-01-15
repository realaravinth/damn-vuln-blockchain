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
//! Client wrapper for p2p communication

use actix_web::client::Client as awc;
use serde::{Deserialize, Serialize};

use crate::asset::{Asset, ReplaceLedger, Stake};
use crate::block::Block;
use crate::config::Config;
use crate::discovery::AddPeer;
use crate::payload::{Peer, Tx, ValidateTx};
use crate::utils::*;
//use crate::logs::SellAsset;

// NOTE these URLs are subject to change
// if tests are failing, come check the URLs
// here
const PEER_ENROLL: &str = "/peers/enroll";
const PEER_DISCOVER_ALL: &str = "/peers/all";
const GET_ALL_ASSETS: &str = "/assets/all";
const SELL_ASSET: &str = "/assets/sell";
const GET_STAKE: &str = "/stake";
const SET_ATTACK: &str = "/attack";
const GET_CHAIN: &str = "/chain/all";
const ADD_BLOCK: &str = "/chain/add";
const SEND_VALIDATOR_TX: &str = "/block/validate";

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
    /// enrolls peer with the auditor enode
    pub async fn peer_enroll(&self, config: &Config) {
        let peer = Peer {
            id: config.peer_id.clone(),
            ip: config.public_ip.clone(),
        };
        let addr = Client::make_uri(&config.auditor_node, PEER_ENROLL);
        self.client
            .post(addr)
            .header("content-type", "application/json")
            .send_json(&peer)
            .await
            .unwrap();
    }

    /// set attack
    pub async fn set_attack(&self, config: &Config) {
        for peer in ["attacker", "victim"].iter() {
            config.info(&format!("Setting {} peer in mode", &peer));
            let attack_peer = get_peer(&config, &format!("{}.batsense.net", peer)).await;
            let addr = Client::make_uri(&attack_peer.ip, SET_ATTACK);
            self.client.post(addr).send().await.unwrap();
        }
    }

    /// get stake for a block
    pub async fn get_stake(&self, peer: GetStake, config: &Config) -> Stake {
        use crate::payload::GetStake as PayloadGetStake;

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
                    for peer in val.iter() {
                        config.network_addr.send(AddPeer(peer.to_owned())).await;
                    }
                    break;
                }
            }
        }
    }

    /// gets asset ledger from auditor node, should be called periodically
    pub async fn get_all_assets(&self, config: &Config) {
        // gets assets from Auditor and replaces assets
        // in local AssetsLedger
        let addr = Client::make_uri(&config.auditor_node, GET_ALL_ASSETS);
        loop {
            if let Ok(mut val) = self.client.get(&addr).send().await {
                config.debug("Asset request success");
                let peers: Result<Vec<Asset>, _> = val.json().await;
                if let Ok(val) = peers {
                    config.debug("Asset deserialization success");
                    config.asset_addr.send(ReplaceLedger(val)).await;
                    break;
                }
            }
        }
    }

    /// send Tx request to validator
    pub async fn send_tx_to_validator(&self, validator: &Peer, payload: &ValidateTx) {
        let addr = Client::make_uri(&validator.ip, SEND_VALIDATOR_TX);

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
    pub async fn get_chain(&self, config: &Config, peer_ip: &str) -> Vec<Block> {
        let addr = Client::make_uri(&peer_ip, GET_CHAIN);

        loop {
            if let Ok(mut val) = self.client.get(&addr).send().await {
                config.debug("Chain dump request success");
                let blocks: Result<Vec<Block>, _> = val.json().await;
                if let Ok(blocks) = blocks {
                    return blocks;
                }
            }
        }
    }

    /// gets list of peers from auditor, should be called periodically
    pub async fn peer_dump(&self, config: &Config) -> Vec<Peer> {
        // gets peers from Auditor and replaces peers
        // in local Network
        let addr = Client::make_uri(&config.auditor_node, PEER_DISCOVER_ALL);
        loop {
            if let Ok(mut val) = self.client.get(&addr).send().await {
                config.debug("Peer discovery request success");
                let peers: Result<Vec<Peer>, _> = val.json().await;
                if let Ok(val) = peers {
                    return val;
                }
            }
        }
    }

    /// gets asset ledger from auditor node, should be called periodically
    pub async fn get_peer_assets(&self, config: &Config, peer: &Peer) {
        // gets assets from Auditor and replaces assets
        // in local AssetsLedger

        let addr = Client::make_uri(&peer.ip, GET_ALL_ASSETS);
        loop {
            if let Ok(mut val) = self.client.get(&addr).send().await {
                config.debug("Asset request success");
                let peers: Result<Vec<Asset>, _> = val.json().await;
                if let Ok(val) = peers {
                    config.debug("Asset deserialization success");
                    config.asset_addr.send(ReplaceLedger(val)).await;
                    break;
                }
            }
        }
    }

    /// send block to peer
    pub async fn send_block_to_peer(&self, config: &Config, peer: &Peer, payload: &Block) {
        let peer_addr = get_peer(&config, &peer.id).await;
        let addr = Client::make_uri(&peer_addr.ip, ADD_BLOCK);
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
}
