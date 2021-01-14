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
use data_encoding::HEXUPPER;
use log::debug;
use sha2::{Digest, Sha256};

use crate::asset::{Asset, AssetLedger, GetAssetInfo, Stake};
use crate::payload::{GetStake, Peer, SellAsset};
use crate::{Client, Config};

/// helper function for generating sha256 hashes
pub fn hasher(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&content);
    let hash = hasher.finalize();
    HEXUPPER.encode(&hash)
}

/// helper function for generating random strings
/// of length = `len`
pub fn get_rand_string(len: usize) -> String {
    use std::iter;

    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(len)
        .collect()
}

/// helper function to get current timesamp
pub fn get_current_time() -> String {
    use chrono::prelude::*;
    Local::now().to_string()
}

///// helper function to get time as string since UNIX_EPOCH
//pub fn timesamp_to_string(timestamp: Timestamp) -> String {
//    unimplemented!()
//}

/// get stake from all peers in network and get the validator peer
pub async fn consensus(config: &Config, block_id: usize, client: &Client) -> Peer {
    use crate::client::GetStake as ClientGetStake;
    use crate::discovery::DumpPeer;
    let mut stake: Vec<(String, Stake)> = Vec::new();
    let peers = config.network_addr.send(DumpPeer).await.unwrap();

    for peer in peers.iter() {
        let client_payload = ClientGetStake {
            block_id,
            peer_id: peer.id.clone(),
        };
        debug!("Requesting stake from peer {}", &peer.id);

        stake.push((
            peer.id.clone(),
            client.get_stake(client_payload, &config).await,
        ));
    }

    // now we have stake of all peers
    // time to calculate validator
    // asset ownership should be
    // verified before calculation
    from_stake_to_validator(&config, stake).await
}

/// get validator peer from stakes of all peers
async fn from_stake_to_validator(config: &Config, all_stakes: Vec<(String, Stake)>) -> Peer {
    use crate::discovery::GetPeer;
    let mut authenticated_stakes: Vec<Asset> = Vec::default();
    for (peer_id, stakes) in all_stakes.iter() {
        for stake in stakes.stake.iter() {
            if let Some(asset) = config
                .asset_addr
                .send(GetAssetInfo(stake.clone()))
                .await
                .unwrap()
            {
                if let Some(owner) = asset.get_owner() {
                    if owner == peer_id {
                        authenticated_stakes.push(asset);
                    }
                }
            }
        }
    }

    debug!("Ownership verified");
    let mut stake_ledger = AssetLedger::new("stake_ledger");
    stake_ledger.assets = authenticated_stakes;
    let validator_peer_id = stake_ledger.choose_validator().unwrap();
    debug!("Validator: {}", &validator_peer_id);

    config
        .network_addr
        .send(GetPeer(validator_peer_id))
        .await
        .unwrap()
        .unwrap()
}

pub async fn check_ownership(config: &Config, owner: &str, asset_id: &str) -> bool {
    let asset_info = config
        .asset_addr
        .send(GetAssetInfo(asset_id.into()))
        .await
        .unwrap()
        .unwrap();
    debug!("Owner: {:?}", asset_info.get_owner());
    if let Some(asset_owner) = asset_info.get_owner() {
        if asset_owner == owner {
            debug!("Ownership verified");
            true
        } else {
            false
        }
    } else {
        false
    }
}

pub async fn get_next_block_id(config: &Config) -> usize {
    use crate::chain::GetLastBlock;
    let current_block = config.chain_addr.send(GetLastBlock).await.unwrap();

    if current_block.get_serial_no().unwrap() == 0 {
        config.init_network_size + 1
    } else {
        current_block.get_serial_no().unwrap() + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::config::Mode;
    use crate::helpers::*;

    #[actix_rt::test]
    async fn consensus_works() {
        let config = init_network(Mode::Normal).await;
        let client = Client::default();
        non_register_bootstrap(&config, &client).await;

        let validator = consensus(&config, 1, &client).await;
        assert_eq!(validator.id, "victim.batsense.net");
    }

    #[actix_rt::test]
    async fn get_next_block_id_works() {
        let config = init_network(Mode::Normal).await;
        assert_eq!(get_next_block_id(&config).await, 4)
    }
}
