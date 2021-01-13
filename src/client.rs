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
use log::{debug, info};
use serde::{Deserialize, Serialize};

use crate::asset::{Asset, ReplaceLedger, Stake};
use crate::config::Config;
use crate::discovery::AddPeer;
use crate::payload::Peer;
//use crate::logs::SellAsset;

// NOTE these URLs are subject to change
// if tests are failing, come check the URLs
// here
const PEER_ENROLL: &str = "/peer/enroll";
const PEER_DISCOVER_ALL: &str = "/peer/discover/all";
const GET_ALL_ASSETS: &str = "/assets/all";
const SELL_ASSET: &str = "/assets/sell";
const GET_STAKE: &str = "/stake";

/// Client wrapper for p2p communication
#[derive(Clone, Default)]
pub struct Client {
    pub client: awc,
}

/// Get stake using client
#[derive(Deserialize, Serialize)]
pub struct GetStake {
    pub block_id: usize,
    pub peer_id: String,
}

impl Client {
    /// enrolls peer with the auditor enode
    pub async fn peer_enroll(&mut self, config: &Config) {
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

    /// get stake for a block
    pub async fn get_stake(&mut self, peer: GetStake, config: &Config) -> Stake {
        use crate::discovery::GetPeer;
        use crate::payload::GetStake as PayloadGetStake;

        let payload = PayloadGetStake {
            block_id: peer.block_id,
        };

        let peer_addr = config
            .network_addr
            .send(GetPeer(peer.peer_id))
            .await
            .unwrap()
            .unwrap();
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
    pub async fn peer_discovery(&mut self, config: &Config) {
        // gets peers from Auditor and replaces peers
        // in local Network
        let addr = Client::make_uri(&config.auditor_node, PEER_DISCOVER_ALL);
        loop {
            if let Ok(mut val) = self.client.get(&addr).send().await {
                debug!("Peer discovery request success");
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
    pub async fn get_all_assets(&mut self, config: &Config) {
        // gets assets from Auditor and replaces assets
        // in local AssetsLedger
        let addr = Client::make_uri(&config.auditor_node, GET_ALL_ASSETS);
        loop {
            if let Ok(mut val) = self.client.get(&addr).send().await {
                debug!("Asset request success");
                let peers: Result<Vec<Asset>, _> = val.json().await;
                if let Ok(val) = peers {
                    debug!("Asset deserialization success");
                    config.asset_addr.send(ReplaceLedger(val)).await;
                    break;
                }
            }
        }
    }
}

///// sells an asset
//    pub async fn sell_asset(&mut self, config: &Config, asset_id: String) {
//        let asset = SellAsset { asset_id };
//
//        unimplemented!("need to implement buy asset workflow");
//        loop {
//            if let Ok(mut val) = self
//                .client
//                .post(format!("{}/{}", &config.auditor_node, SELL_ASSET))
//                .header("content-type", "application/json")
//                .send_json(&asset)
//                .await
//            {
//                // TODO
//                //            let peers: Result<Vec<Peer>, _> = val.json().await;
//                //            if let Ok(val) = peers {
//                //                for peer in val.iter() {
//                //                    config.network_addr.send(AddPeer(peer.to_owned())).await;
//                //                }
//                //                break;
//                //            }
//            }
//        }
//    }
//}
