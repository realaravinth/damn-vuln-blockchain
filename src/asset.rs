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

use std::fmt::{Display, Formatter, Result};

use actix::prelude::*;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
pub struct Asset {
    name: String,
    value: usize,
    hash: String,
    owner: Option<String>,
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

    /// get owner of the asset
    pub fn get_owner(&self) -> &Option<String> {
        &self.owner
    }
}

#[derive(Deserialize, Default, Serialize, Clone, Debug)]
pub struct AssetLedger {
    pub assets: Vec<Asset>,
}

impl AssetLedger {
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
pub struct ReplaceLedger(Vec<Asset>);

impl Handler<InitNetwork> for AssetLedger {
    type Result = MessageResult<InitNetwork>;

    fn handle(&mut self, msg: InitNetwork, _ctx: &mut Self::Context) -> Self::Result {
        let length = self.assets.len();
        let mut assets_per_peer = length / msg.network_size;
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
}
