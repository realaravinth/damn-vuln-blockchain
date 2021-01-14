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
//! Assets are objects that can be transacted on the blockchain
//!
//! The easiest way to manage [Asset] with te [AssetLedger] actor
//! # [AssetLedger] Messages:
//! - [ChangeAssetOwner]: Changes an asset's owner
//! - [InitNetwork]: Initializes assets to peers in the network
//! - [GetAssetInfo]: Get an asset's info
//! - [DumpLedger]: Dump the entire asset ledger
//! - [ReplaceLedger]: Replace the current ledger with another ledger, useful when
//! - [ChooseValidator]: Choose validator based on coinage
//! - [GetPeerAssets]: Get all the assets belonging to a peer
//! - [SetStake]: Set stake for a block creation
//! - [GetStake]: Get stake for a block  ID
//! synchronising state

use std::fmt::{Display, Formatter, Result};

use actix::prelude::*;
use derive_builder::Builder;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::payload::GetStake as PayloadGetStake;

/// /// [Asset]s are objects that can be transacted on the blockchain
#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
pub struct Asset {
    name: String,
    value: usize,
    hash: String,
    owner: Option<String>,
    /// The last transaction where this asset was used
    /// This value should be the same as [Block.get_serial_no()]
    /// transaction 0 = never been used
    last_transaction: usize,
}

impl Display for Asset {
    #[cfg(not(tarpaulin_include))]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let owner = if let Some(val) = self.get_owner() {
            val
        } else {
            "Fresh out of mint"
        };
        write!(
            f,
            "Hash:{}\nName: {}\nValue: {}\nOwner: {}",
            self.get_hash(),
            self.get_name(),
            self.get_value(),
            owner
        )
    }
}

impl Asset {
    /// create new asset
    pub fn new(name: &str, value: usize) -> Self {
        use super::utils::{get_rand_string, hasher};

        let hash = hasher(&format!("{}-{}{}", get_rand_string(10), &name, &value));
        Asset {
            name: name.into(),
            value,
            owner: None,
            hash,
            last_transaction: 0,
        }
    }

    /// get name of the asset
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// get hash of the asset
    pub fn get_hash(&self) -> &str {
        &self.hash
    }

    /// get value/price of the asset
    pub fn get_value(&self) -> usize {
        self.value
    }

    /// set owner of the asset
    pub fn set_owner(&mut self, owner: &str) {
        self.owner = Some(owner.into());
    }

    /// set last transaction validated by the asset
    pub fn set_last_transaction(&mut self, last_transaction: usize) {
        self.last_transaction = last_transaction;
    }

    /// get owner of the asset
    pub fn get_owner(&self) -> &Option<String> {
        &self.owner
    }

    /// get last transaction validated by the asset
    pub fn get_last_transaction(&self) -> usize {
        self.last_transaction
    }
}

/// represents the world(full network) state of [Asset]
/// # [AssetLedger] Messages:
/// - [ChangeAssetOwner]: Changes an asset's owner
/// - [InitNetwork]: Initializes assets to peers in the network
/// - [GetAssetInfo]: Get an asset's info
/// - [DumpLedger]: Dump the entire asset ledger
/// - [ReplaceLedger]: Replace the current ledger with another ledger, useful when
/// - [ChooseValidator]: Choose validator based on coinage
/// - [GetPeerAssets]: Get all the assets belonging to a peer
/// - [SetStake]: Set stake for a block creation
/// - [GetStake]: Get stake for a block  ID
/// synchronising state
#[derive(Deserialize, Serialize, Default, Clone, Debug)]
pub struct AssetLedger {
    pub assets: Vec<Asset>,
    pub stake: Vec<Stake>,
    pub peer_id: String,
}

/// represents the stake each peer is willing to send
/// for every block creation.
/// - `block_id` represents the future block ID that
/// this stake is being set for
/// - stake represnts the assets' hashes
#[derive(Deserialize, Builder, Default, Serialize, Clone, Debug)]
pub struct Stake {
    pub block_id: usize,
    pub stake: Vec<String>,
}

impl AssetLedger {
    pub fn new(peer_id: &str) -> Self {
        AssetLedger {
            assets: Vec::default(),
            stake: Vec::default(),
            peer_id: peer_id.into(),
        }
    }

    fn get_peer_assets(&self, peer_id: &str) -> Vec<Asset> {
        let mut payload: Vec<Asset> = Vec::new();
        self.assets.iter().for_each(|asset| {
            if asset.get_owner().is_some() && asset.get_owner().as_ref().unwrap() == peer_id {
                payload.push(asset.clone());
            }
        });

        payload
    }

    fn default_stake(&self) -> Vec<String> {
        let assets_for_me = self.get_peer_assets(&self.peer_id);

        let mut stake_id = Vec::new();
        assets_for_me.iter().for_each(|asset| {
            stake_id.push(asset.get_hash().to_owned());
        });
        stake_id
    }

    // get the current number of peers assigned
    fn peers_currently_assigned(&self) -> usize {
        let mut peers_currently_assigned: Vec<&str> = Vec::new();
        self.assets.iter().for_each(|asset| {
            if let Some(owner) = asset.get_owner() {
                let peer_check = peers_currently_assigned.iter().find(|peer_id| {
                    if **peer_id == owner {
                        true
                    } else {
                        false
                    }
                });

                if peer_check.is_none() {
                    peers_currently_assigned.push(owner);
                }
            };
        });
        peers_currently_assigned.len()
    }

    /// generates a bunch of fake assets
    pub fn generate(peer_id: &str) -> AssetLedger {
        debug!("Gerating assets");
        let mut ledger = AssetLedger {
            assets: Vec::new(),
            stake: Vec::new(),
            peer_id: peer_id.into(),
        };

        ledger.assets.push(Asset::new("les Escaldes", 100));
        ledger.assets.push(Asset::new("Andorra la Vella", 100));
        ledger.assets.push(Asset::new("Umm al Qaywayn", 100));
        ledger.assets.push(Asset::new("Ras al-Khaimah", 100));
        ledger.assets.push(Asset::new("Khawr Fakkān", 100));
        ledger.assets.push(Asset::new("Dubai", 100));
        ledger.assets.push(Asset::new("Dibba Al-Fujairah", 100));
        ledger.assets.push(Asset::new("Dibba Al-Hisn", 100));
        ledger.assets.push(Asset::new("Sharjah", 100));
        ledger.assets.push(Asset::new("Ar Ruways", 100));
        ledger.assets.push(Asset::new("Al Fujayrah", 100));
        ledger.assets.push(Asset::new("Al Ain", 100));
        ledger.assets.push(Asset::new("Ajman", 100));
        ledger.assets.push(Asset::new("Adh Dhayd", 100));
        ledger.assets.push(Asset::new("Abu Dhabi", 100));
        ledger
    }

    /// chooses validator based on proof of stake
    pub fn choose_validator(&self) -> Option<String> {
        // Vec<asset_count, (coinage, str)>
        // asset_count = directly proportional to stake
        // coinage(here validated transaction's ID, ID is serial) inversly proportional
        // no, coinage = getLastTransaction - coinage stored in asset
        // stake =
        //
        // stake = (assets.iter().max()(gets the latest transaction) - asset.get_coinage()(get's
        // the transaction ID in which the asset was used)) * number of assets
        // the peer has (https://en.wikipedia.org/wiki/Proof_of_stake#Coin_age-based_selection)
        //let mut counter: Vec<(usize, (usize, &str))> = Vec::new();

        let mut counter: Vec<(usize, &str)> = Vec::new();
        // problem statement:
        // - all assets shuold be accounted for
        // - coinage must be compared
        // - and the ownser with the most oldest coinage must be chosen

        // solution:
        // - sum all coinages of all assets that an ownwer has
        // - owner with least sum becomes validator

        // get latest transaction
        // unwrap is fine here because last_transaction is set at initialization
        let latest_transaction = self
            .assets
            .iter()
            .max_by_key(|asset| asset.get_last_transaction())
            .unwrap()
            .get_last_transaction();

        // calculate coinage
        self.assets.iter().for_each(|asset| {
            // it's possible that the minted assets are not yet assigned
            if let Some(owner) = asset.get_owner() {
                // unwrap is okay jere because get_owner() and get_coinage()
                // are in sync when retruning None and it's checked for
                let asset_last_transaction = asset.get_last_transaction();
                let coinage = latest_transaction - asset_last_transaction;

                // flag to check if an asset has been counted
                let mut counted_flag = false;

                // checking if owner already exists in counter
                // datastructure
                for (coinage_sum, peer_id) in counter.iter_mut() {
                    if peer_id == owner {
                        *coinage_sum += coinage;
                        counted_flag = true;
                        break;
                    }
                }
                if !counted_flag {
                    counter.push((coinage, owner));
                }
            }
        });

        // return the minimum value(when coimage_sum minimum, coinage maximum)
        // present in the counter datastructure.
        if let Some((_, peer_id)) = counter.iter().max() {
            Some(peer_id.to_string())
        } else {
            None
        }
    }
}

impl Actor for AssetLedger {
    type Context = Context<Self>;
}

///// Mints assets, should only be called when the blockchain is initiated
///// and should only be executed by the first peer on the network
//#[derive(Message)]
//#[rtype(result = "()")]
//pub struct MintAssetLedger;

/// Changes owner of asset `ChangeOwner.0` to  `ChangeOwner.1`
#[derive(Message, Builder)]
#[rtype(result = "()")]
pub struct ChangeAssetOwner {
    pub asset_id: String,
    pub new_owner: String,
}

/// Initializes assets to peers in the network
/// AssetLedger` will automatically devide available assets
/// please note that this call will result in error if it's called
/// after all the assets' owners are changed from None to Some(val)
#[derive(Message, Builder)]
#[rtype(result = "()")]
pub struct InitNetwork {
    pub network_size: usize,
    pub peer_id: String,
}

/// Get asset info of `GetAssetInfo.0`
#[derive(Message)]
#[rtype(result = "Option<Asset>")]
pub struct GetAssetInfo(pub String);

/// Dumps entire ledger
#[derive(Message)]
#[rtype(result = "Vec<Asset>")]
pub struct DumpLedger;

/// Replaces asset ledger
/// Useful when forking
#[derive(Message)]
#[rtype(result = "()")]
pub struct ReplaceLedger(pub Vec<Asset>);

/// Get asset info of `GetAssetInfo.0`
#[derive(Message)]
#[rtype(result = "Option<String>")]
pub struct ChooseValidator;

/// Get assets belonging to a peer
#[derive(Message)]
#[rtype(result = "Vec<Asset>")]
pub struct GetPeerAssets(pub String);

/// Set stake for a block ID
#[derive(Builder, PartialEq, Clone, Message)]
#[rtype(result = "()")]
pub struct SetStake {
    pub block_id: usize,
    pub peer_id: String,
    pub stake: Vec<String>,
}

/// Get stake for a block ID
#[derive(Deserialize, Serialize, Message)]
#[rtype(result = "Stake")]
pub struct GetStake(pub usize);

impl From<PayloadGetStake> for GetStake {
    fn from(msg: PayloadGetStake) -> Self {
        GetStake(msg.block_id)
    }
}

impl Handler<InitNetwork> for AssetLedger {
    type Result = MessageResult<InitNetwork>;

    fn handle(&mut self, msg: InitNetwork, _ctx: &mut Self::Context) -> Self::Result {
        let length = self.assets.len();
        let mut assets_per_peer = length / msg.network_size;

        //let peers_currently_assigned = self.assets.sort_by(|asset| {
        //    if let Some(owner) = asset.get_owner() {
        //        owner
        //    } else {
        //        "unowned"
        //});

        let current_transaction = self.peers_currently_assigned() + 1;

        for i in 0..length {
            // unwrap is okay here as I'm only `get_mut()`ing over the
            // the length of `self.assets`
            let asset = self.assets.get_mut(i).unwrap();

            // number of assets that need to be modified are
            // controlled by this:
            if assets_per_peer > 0 {
                // assets that already have an owner shouldn't be effected
                if asset.get_owner().is_none() {
                    asset.set_owner(&msg.peer_id);
                    // initializing coinage to 0(ready for use)
                    asset.set_last_transaction(current_transaction);
                    assets_per_peer -= 1;
                }
            }
        }
        MessageResult(())
    }
}

impl Handler<DumpLedger> for AssetLedger {
    type Result = MessageResult<DumpLedger>;

    fn handle(&mut self, _msg: DumpLedger, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.assets.clone())
    }
}

impl Handler<GetAssetInfo> for AssetLedger {
    type Result = MessageResult<GetAssetInfo>;

    fn handle(&mut self, msg: GetAssetInfo, _ctx: &mut Self::Context) -> Self::Result {
        let mut iter = self.assets.iter();
        loop {
            if let Some(val) = iter.next() {
                if val.get_hash() == msg.0 {
                    return MessageResult(Some(val.clone()));
                }
            } else {
                break;
            }
        }
        MessageResult(None)
    }
}

impl Handler<ChangeAssetOwner> for AssetLedger {
    type Result = ();

    fn handle(&mut self, msg: ChangeAssetOwner, _ctx: &mut Self::Context) -> Self::Result {
        let mut target: Option<usize> = None;
        for (index, asset) in self.assets.iter().enumerate() {
            if asset.get_hash() == msg.asset_id {
                target = Some(index);
                break;
            }
        }

        if let Some(index) = target {
            self.assets
                .get_mut(index)
                .unwrap()
                .set_owner(&msg.new_owner);
        };
    }
}

impl Handler<ReplaceLedger> for AssetLedger {
    type Result = ();

    fn handle(&mut self, msg: ReplaceLedger, _ctx: &mut Self::Context) -> Self::Result {
        self.assets = msg.0;
        debug!("Replaced AssetLedger for peer: {}", &self.peer_id);
    }
}

impl Handler<ChooseValidator> for AssetLedger {
    type Result = MessageResult<ChooseValidator>;

    fn handle(&mut self, _msg: ChooseValidator, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.choose_validator())
    }
}

impl Handler<GetPeerAssets> for AssetLedger {
    type Result = MessageResult<GetPeerAssets>;

    fn handle(&mut self, msg: GetPeerAssets, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.get_peer_assets(&msg.0))
    }
}

impl Handler<SetStake> for AssetLedger {
    type Result = ();

    fn handle(&mut self, msg: SetStake, _ctx: &mut Self::Context) -> Self::Result {
        let assets = self.get_peer_assets(&msg.peer_id);

        let mut correct_stake: Vec<String> = Vec::new();

        // checks if the asset hashes received are
        // indeed owned by the peer ID received
        // and filters assets accordingly
        // Ideally, when a match isn't found
        // an error must be returned
        for stake in msg.stake.iter() {
            let hash = assets.iter().find(|asset| {
                if asset.get_hash() == stake {
                    true
                } else {
                    false
                }
            });
            if hash.is_some() {
                correct_stake.push(hash.unwrap().get_hash().to_owned())
            }
        }

        let stake = StakeBuilder::default()
            .block_id(msg.block_id)
            .stake(correct_stake)
            .build()
            .unwrap();

        self.stake.push(stake);
    }
}

impl Handler<GetStake> for AssetLedger {
    type Result = MessageResult<GetStake>;

    fn handle(&mut self, msg: GetStake, _ctx: &mut Self::Context) -> Self::Result {
        let stake = self
            .stake
            .iter()
            .find(|stake| if stake.block_id == msg.0 { true } else { false });
        if stake.is_some() {
            MessageResult(stake.unwrap().to_owned())
        } else {
            debug!("GetStake is empty, generating default stake");
            let stake = StakeBuilder::default()
                .block_id(msg.0)
                .stake(self.default_stake())
                .build()
                .unwrap();

            self.stake.push(stake.clone());
            println!("{:#?}", self.default_stake());
            MessageResult(stake)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn mint_asset_works() {
        let mut asset = Asset::new("Delhi", 100);
        assert_eq!(asset.get_name(), "Delhi");
        assert_eq!(asset.get_owner(), &None);
        assert_eq!(asset.get_value(), 100);

        let new_owner = "Me".to_string();
        asset.set_owner(&new_owner);
        assert_eq!(asset.get_owner(), &Some(new_owner));
        asset.set_last_transaction(1);
        assert_eq!(asset.get_last_transaction(), 1);
    }

    #[actix_rt::test]
    async fn asset_ledger_init_network_works() {
        let peer_id = "me";
        let asset_ledger = AssetLedger::generate(&peer_id);
        let assert_ledger_addr = asset_ledger.clone().start();
        let network_size: usize = 3;
        let msg = InitNetworkBuilder::default()
            .network_size(network_size)
            .peer_id(peer_id.into())
            .build()
            .unwrap();
        assert_ledger_addr.send(msg).await.unwrap();

        let dump = assert_ledger_addr.send(DumpLedger).await.unwrap();

        let length = dump.len();
        let assets_per_peer = length / network_size;

        let mut asset_ledger_per_peer_state = 0;

        for i in dump.iter() {
            if i.get_owner().is_some() {
                // ownership is verified here if ownder != "me", then the
                // below statement should panic
                assert_eq!(
                    i.get_owner().as_ref().unwrap(),
                    peer_id,
                    "asset ownder rightly assigned"
                );
                asset_ledger_per_peer_state += 1;
            }
        }

        assert_eq!(
            assets_per_peer, asset_ledger_per_peer_state,
            "assets per peer satisfied, no over allocation, no under allocation"
        );
    }

    #[actix_rt::test]
    async fn asset_ledger_actor_works() {
        let peer_id = "Me";
        let asset_ledger = AssetLedger::generate(peer_id.into());
        let assert_ledger_addr = asset_ledger.clone().start();

        let dump = assert_ledger_addr.send(DumpLedger).await.unwrap();
        let iter = dump.iter().zip(asset_ledger.assets.iter());

        iter.for_each(|(a, b)| assert_eq!(a, b, "AssetLedger dump test"));

        let hash = asset_ledger.assets.get(2).unwrap().get_hash();
        assert_eq!(
            hash,
            assert_ledger_addr
                .send(GetAssetInfo(hash.into()))
                .await
                .unwrap()
                .unwrap()
                .get_hash(),
            "AssetLedger GetAssetInfo test"
        );

        let change_ownser_message = ChangeAssetOwnerBuilder::default()
            .asset_id(hash.into())
            .new_owner(peer_id.into())
            .build()
            .unwrap();
        assert_ledger_addr
            .send(change_ownser_message)
            .await
            .unwrap();

        assert_eq!(
            &Some(peer_id.into()),
            assert_ledger_addr
                .send(GetAssetInfo(hash.into()))
                .await
                .unwrap()
                .unwrap()
                .get_owner(),
            "AssetLedger ChangeOwner test"
        );

        let fork_asset_ledger_addr = asset_ledger.clone().start();
        let new_dump = assert_ledger_addr.send(DumpLedger).await.unwrap();
        fork_asset_ledger_addr
            .send(ReplaceLedger(new_dump.clone()))
            .await
            .unwrap();

        let forked_dump = fork_asset_ledger_addr.send(DumpLedger).await.unwrap();
        forked_dump
            .iter()
            .zip(new_dump.iter())
            .for_each(|(a, b)| assert_eq!(a, b, "AssetLedger replace check"));
    }

    #[actix_rt::test]
    async fn choose_validator_works() {
        let peer_id = "me";
        let asset_ledger = AssetLedger::generate(peer_id);
        let assert_ledger_addr = asset_ledger.clone().start();
        let network_size: usize = 3;

        let mut msg = InitNetworkBuilder::default()
            .network_size(network_size)
            .peer_id(peer_id.into())
            .build()
            .unwrap();
        assert_ledger_addr.send(msg).await.unwrap();
        // testng ChooseValidator
        assert_eq!(
            assert_ledger_addr.send(ChooseValidator).await.unwrap(),
            Some(peer_id.into()),
            "ChooseValidator works"
        );

        msg = InitNetworkBuilder::default()
            .network_size(network_size)
            .peer_id("you".into())
            .build()
            .unwrap();
        assert_ledger_addr.send(msg).await.unwrap();
        // testng ChooseValidator
        assert_eq!(
            assert_ledger_addr.send(ChooseValidator).await.unwrap(),
            Some("me".into()),
            "ChooseValidator works"
        );

        msg = InitNetworkBuilder::default()
            .network_size(network_size)
            .peer_id("us".into())
            .build()
            .unwrap();
        assert_ledger_addr.send(msg).await.unwrap();
        // testng ChooseValidator
        assert_eq!(
            assert_ledger_addr.send(ChooseValidator).await.unwrap(),
            Some("me".into()),
            "ChooseValidator works"
        );
    }

    #[actix_rt::test]
    async fn get_peer_assets_works() {
        let peer_id = "me";
        let asset_addr = AssetLedger::generate(peer_id).start();
        let mut assets_for_me = asset_addr.send(GetPeerAssets("Me".into())).await.unwrap();

        let network_size: usize = 3;

        assert_eq!(assets_for_me.len(), 0);

        let msg = InitNetworkBuilder::default()
            .network_size(network_size)
            .peer_id(peer_id.into())
            .build()
            .unwrap();
        asset_addr.send(msg).await.unwrap();
        assets_for_me = asset_addr
            .send(GetPeerAssets(peer_id.into()))
            .await
            .unwrap();
        assert_eq!(assets_for_me.len(), 5);
    }

    #[actix_rt::test]
    async fn stake_works() {
        let peer_id = "me";
        let asset_addr = AssetLedger::generate(peer_id).start();
        let mut assets_for_me = asset_addr.send(GetPeerAssets("Me".into())).await.unwrap();

        let network_size: usize = 3;

        assert_eq!(assets_for_me.len(), 0);

        let msg = InitNetworkBuilder::default()
            .network_size(network_size)
            .peer_id(peer_id.into())
            .build()
            .unwrap();
        asset_addr.send(msg).await.unwrap();
        assets_for_me = asset_addr
            .send(GetPeerAssets(peer_id.into()))
            .await
            .unwrap();
        let mut stake_id = Vec::new();

        let mut count = 2;

        assets_for_me.iter().for_each(|asset| {
            if count > 0 {
                stake_id.push(asset.get_hash().to_owned());
                count -= 1;
            }
        });

        let set_stake_msg = SetStakeBuilder::default()
            .block_id(5)
            .stake(stake_id)
            .peer_id(peer_id.into())
            .build()
            .unwrap();

        // checking default stake
        let mut default_stake_id: Vec<String> = Vec::new();
        assets_for_me.iter().for_each(|asset| {
            default_stake_id.push(asset.get_hash().to_owned());
        });

        let stake = asset_addr.send(GetStake(4)).await.unwrap();

        assert_eq!(stake.block_id, 4);
        assert_eq!(stake.stake, default_stake_id);

        // checking custom stake
        asset_addr.send(set_stake_msg.clone()).await.unwrap();
        let stake = asset_addr.send(GetStake(5)).await.unwrap();

        assert_eq!(stake.block_id, set_stake_msg.block_id);
        assert_eq!(stake.stake, set_stake_msg.stake);

        asset_addr.send(set_stake_msg.clone()).await.unwrap();
    }

    #[test]
    fn peers_currently_assigned_works() {
        let peer_id = "me";
        let mut assets = AssetLedger::generate(peer_id);
        let assets_per_peer = 3;
        assert_eq!(assets.peers_currently_assigned(), 0);
        assign_assets(&mut assets, peer_id, assets_per_peer);
        assert_eq!(assets.peers_currently_assigned(), 1);
        assign_assets(&mut assets, "you", assets_per_peer);

        assert_eq!(assets.peers_currently_assigned(), 2);

        assign_assets(&mut assets, "use", assets_per_peer);

        assert_eq!(assets.peers_currently_assigned(), 3);
    }

    fn assign_assets(assets: &mut AssetLedger, peer_id: &str, change_assets: usize) {
        let length = assets.assets.len();
        let mut assets_per_peer = change_assets.clone();
        for i in 0..length {
            // unwrap is okay here as I'm only `get_mut()`ing over the
            // the length of `self.assets`
            let asset = assets.assets.get_mut(i).unwrap();

            // number of assets that need to be modified are
            // controlled by this:
            if assets_per_peer > 0 {
                // assets that already have an owner shouldn't be effected
                if asset.get_owner().is_none() {
                    asset.set_owner(&peer_id);
                    // initializing coinage to 0(ready for use)
                    assets_per_peer -= 1;
                }
            }
        }
    }

    #[test]
    fn default_stake_works() {
        let peer_id = "me";
        let mut assets = AssetLedger::generate(peer_id);
        let assets_per_peer = 3;
        assign_assets(&mut assets, peer_id, assets_per_peer);
        let assets_for_me = assets.get_peer_assets(&peer_id);
        let mut stake = Vec::new();
        assets_for_me.iter().for_each(|asset| {
            if asset.get_owner().is_some() {
                stake.push(asset.get_hash().to_owned());
            }
        });

        assert_eq!(stake, assets.default_stake())
    }
}
