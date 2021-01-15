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
use sha2::{Digest, Sha256};

use crate::asset::{Asset, AssetLedger, GetAssetInfo, Stake};
use crate::block::Block;
use crate::payload::Peer;
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
        config.debug(&format!("Requesting stake from peer {}", &peer.id));

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

/// get peer utility
pub async fn get_peer(config: &Config, peer_id: &str) -> Peer {
    use crate::discovery::GetPeer;

    config
        .network_addr
        .send(GetPeer(peer_id.into()))
        .await
        .unwrap()
        .unwrap()
}

/// get validator peer from stakes of all peers
async fn from_stake_to_validator(config: &Config, all_stakes: Vec<(String, Stake)>) -> Peer {
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

    config.debug("Ownership verified");
    let mut stake_ledger = AssetLedger::new("stake_ledger");
    stake_ledger.assets = authenticated_stakes;
    let validator_peer_id = stake_ledger.choose_validator().unwrap();
    config.debug(&format!("Validator: {}", &validator_peer_id));

    get_peer(&config, &validator_peer_id).await
}

/// check ownsership utility
pub async fn check_ownership(config: &Config, owner: &str, asset_id: &str) -> bool {
    let asset_info = config
        .asset_addr
        .send(GetAssetInfo(asset_id.into()))
        .await
        .unwrap()
        .unwrap();
    config.debug(&format!("Owner: {:?}", asset_info.get_owner()));
    if let Some(asset_owner) = asset_info.get_owner() {
        if asset_owner == owner {
            config.debug("Ownership verified");
            true
        } else {
            false
        }
    } else {
        false
    }
}

/// get next block ID utility
pub async fn get_next_block_id(config: &Config) -> usize {
    use crate::chain::GetLastBlock;
    let current_block = config.chain_addr.send(GetLastBlock).await.unwrap();

    if current_block.get_serial_no().unwrap() == 0 {
        config.init_network_size + 1
    } else {
        current_block.get_serial_no().unwrap() + 1
    }
}

/// add block utility. Performs the following steps:
/// 1. Change asset ownership
/// 2. mutate validation assets and sold last transaction
/// 3. add block to chain
pub async fn add_block_runner(config: &Config, client: &Client, block: &Block) {
    use crate::asset::{ChangeAssetOwnerBuilder, SetLastTransationBuilder};
    use crate::chain::AddBlock;
    use crate::client::GetStake as ClientGetStake;

    let next_block_id = get_next_block_id(&config).await;

    // changing asset ownsership
    config.debug(&format!(
        "Chainging ownership of asset: {}",
        block.get_asset_id().unwrap()
    ));
    let change_ownsership_msg = ChangeAssetOwnerBuilder::default()
        .asset_id(block.get_asset_id().unwrap().into())
        .new_owner(block.get_rx().unwrap().into())
        .build()
        .unwrap();
    config.asset_addr.send(change_ownsership_msg).await.unwrap();

    // changing coinage of the asset transacted
    config.debug(&format!(
        "Chainging coinage of asset: {}",
        block.get_asset_id().unwrap()
    ));
    let change_tx_msg = SetLastTransationBuilder::default()
        .tx(next_block_id)
        .asset_id(block.get_asset_id().unwrap().into())
        .build()
        .unwrap();
    config.asset_addr.send(change_tx_msg).await.unwrap();

    // changing coinage of the assets staked by the validator
    let client_payload = ClientGetStake {
        peer_id: block.get_validator().unwrap().into(),
        block_id: next_block_id,
    };
    let validator_stakes = client.get_stake(client_payload, &config).await;
    for asset_id in validator_stakes.stake.iter() {
        config.debug(&format!("Chainging coinage of asset: {}", &asset_id));

        let change_tx_msg = SetLastTransationBuilder::default()
            .tx(next_block_id)
            .asset_id(asset_id.into())
            .build()
            .unwrap();
        config.asset_addr.send(change_tx_msg).await.unwrap();
    }

    // adding block to chain
    config.info(&format!("Adding block {} to chain", block.get_hash()));
    config
        .chain_addr
        .send(AddBlock(block.to_owned(), config.init_network_size))
        .await
        .unwrap()
        .unwrap();
}

/// broadcast block to all peers
pub async fn broadcast_block(config: &Config, client: &Client, block: &Block) {
    use crate::discovery::DumpPeer;
    let peers = config.network_addr.send(DumpPeer).await.unwrap();
    for peer in peers.iter() {
        if peer.id != config.peer_id {
            config.debug(&format!(
                "Broadcasting block {} to peer {}",
                &block.get_hash(),
                &peer.id
            ));

            client.send_block_to_peer(&config, &peer, &block).await;
        }
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
