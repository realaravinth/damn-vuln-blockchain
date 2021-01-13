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
//! synchronising state

use std::fmt::{Display, Formatter, Result};

use actix::prelude::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

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
/// synchronising state

#[derive(Deserialize, Default, Serialize, Clone, Debug)]
pub struct AssetLedger {
    pub assets: Vec<Asset>,
}

impl AssetLedger {
    // get the current number of peers assigned
    fn peers_currently_assigned(&self) -> usize {
        let mut peers_currently_assigned: Vec<&str> = Vec::new();
        self.assets.iter().for_each(|asset| {
            let owner = match asset.get_owner() {
                Some(val) => val,
                None => "Notassigned",
            };
            let peer_check =
                peers_currently_assigned.iter().find(
                    |peer_id| {
                        if **peer_id == owner {
                            true
                        } else {
                            false
                        }
                    },
                );

            if peer_check.is_none() {
                peers_currently_assigned.push(owner);
            }
        });
        peers_currently_assigned.len() - 1
    }

    /// generates a bunch of fake assets
    pub fn generate() -> AssetLedger {
        let mut ledger = AssetLedger { assets: Vec::new() };

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
        let mut payload: Vec<Asset> = Vec::new();
        self.assets.iter().for_each(|asset| {
            if asset.get_owner().is_some() && asset.get_owner().as_ref().unwrap() == &msg.0 {
                payload.push(asset.clone());
            }
        });

        MessageResult(payload.to_owned())
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
        let asset_ledger = AssetLedger::generate();
        let assert_ledger_addr = asset_ledger.clone().start();
        let network_size: usize = 3;
        let msg = InitNetworkBuilder::default()
            .network_size(network_size)
            .peer_id("me".into())
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
                    "me",
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
        let asset_ledger = AssetLedger::generate();
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
            .new_owner("Me".into())
            .build()
            .unwrap();
        assert_ledger_addr
            .send(change_ownser_message)
            .await
            .unwrap();

        assert_eq!(
            &Some("Me".to_string()),
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
        let asset_ledger = AssetLedger::generate();
        let assert_ledger_addr = asset_ledger.clone().start();
        let network_size: usize = 3;

        let mut msg = InitNetworkBuilder::default()
            .network_size(network_size)
            .peer_id("me".into())
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
        let asset_addr = AssetLedger::generate().start();
        let mut assets_for_me = asset_addr.send(GetPeerAssets("Me".into())).await.unwrap();

        let network_size: usize = 3;

        assert_eq!(assets_for_me.len(), 0);

        let msg = InitNetworkBuilder::default()
            .network_size(network_size)
            .peer_id("Me".into())
            .build()
            .unwrap();
        asset_addr.send(msg).await.unwrap();
        assets_for_me = asset_addr.send(GetPeerAssets("Me".into())).await.unwrap();
        assert_eq!(assets_for_me.len(), 5);
    }

    #[test]
    fn peers_currently_assigned_works() {
        let mut assets = AssetLedger::generate();
        let assets_per_peer = 3;
        assert_eq!(assets.peers_currently_assigned(), 0);
        assign_assets(&mut assets, "me", assets_per_peer);
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
}
