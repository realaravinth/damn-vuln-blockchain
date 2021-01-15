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
use actix::prelude::*;
use log::{debug, info};

use crate::asset::AssetLedger;
use crate::chain::Chain;
use crate::discovery::Network;
use crate::Client;

#[derive(Clone)]
pub struct Config {
    pub mode_addr: Addr<ModeActor>,
    pub peer_id: String,
    pub public_ip: String,
    pub auditor_node: String,
    pub asset_addr: Addr<AssetLedger>,
    pub chain_addr: Addr<Chain>,
    pub tampered_chain_addr: Addr<Chain>,
    pub tampered_asset_addr: Addr<AssetLedger>,
    pub network_addr: Addr<Network>,
    pub init_network_size: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Mode {
    Auditor,
    /// set Attacker = true when
    /// mounting attack, i.e, when
    /// maximum stake is required
    Attacker(bool),
    /// set Victim = true when
    /// under attack
    /// This simulates DoS by
    /// not making any changes
    /// to local ledgers and returning
    /// empty stake
    Victim(bool),
    Normal,
}

impl Config {
    #[cfg(not(tarpaulin_include))]
    pub fn new() -> Self {
        Config::cli()
    }

    pub async fn get_asset_ledger(&self) -> Addr<AssetLedger> {
        if self.mode_addr.send(GetMode).await.unwrap() == Mode::Attacker(true) {
            self.tampered_asset_addr.clone()
        } else {
            self.asset_addr.clone()
        }
    }

    pub async fn get_chain_addr(&self) -> Addr<Chain> {
        if self.mode_addr.send(GetMode).await.unwrap() == Mode::Attacker(true) {
            self.tampered_chain_addr.clone()
        } else {
            self.chain_addr.clone()
        }
    }

    /// debug logging wrapper
    #[cfg(not(tarpaulin_include))]
    pub fn debug(&self, msg: &str) {
        debug!("[{}]: {}", &self.peer_id, msg);
    }

    pub fn fork_chain(&self) -> Self {
        Config {
            peer_id: self.peer_id.clone(),
            mode_addr: self.mode_addr.clone(),
            asset_addr: self.tampered_asset_addr.clone(),
            tampered_chain_addr: self.tampered_chain_addr.clone(),
            tampered_asset_addr: self.tampered_asset_addr.clone(),
            chain_addr: self.tampered_chain_addr.clone(),
            network_addr: self.network_addr.clone(),
            init_network_size: self.init_network_size,
            auditor_node: self.auditor_node.clone(),
            public_ip: self.public_ip.clone(),
        }
    }

    /// info logging wrapper
    #[cfg(not(tarpaulin_include))]
    pub fn info(&self, msg: &str) {
        info!("[{}]: {}", &self.peer_id, msg);
    }

    #[cfg(not(tarpaulin_include))]
    pub async fn bootstrap(&self) {
        use crate::chain::ReplaceChain;
        if self.mode_addr.send(GetMode).await.unwrap() != Mode::Auditor {
            self.info("Bootstrapping node");
            let client = Client::default();
            self.info("Enrolling peer");
            client.peer_enroll(&self).await;
            self.info("Discovering peers in network");
            client.peer_discovery(&self).await;
            self.info("Bootstrapping assets");
            client.get_all_assets(&self).await;
            let chain = client.get_chain(&self, &self.auditor_node).await;
            self.chain_addr.send(ReplaceChain(chain)).await;
        }
    }

    #[cfg(not(tarpaulin_include))]
    fn cli() -> Self {
        use clap::{App, Arg};
        let matches = App::new("Damn Vulnerable Blockchain")
            .version("0.1")
            .author("Aravinth Manivannan <realaravinth@batsense.net>")
            .about("A bloody vulnerable blockchain implementation")
            .arg(
                Arg::with_name("public_ip")
                    .help("set public IP")
                    .short("-i")
                    .long("--public-ip")
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("peer_id")
                    .help("set peer name")
                    .short("-n")
                    .long("--name")
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("network_size")
                    .help("set intial network size")
                    .short("-s")
                    .long("--network-size")
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("mode")
                    .help("available modes:\n\tauditor\n\tnormal\n\tattacker\n\tvictim ")
                    .short("-m")
                    .long("--mode")
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("auditor")
                    .help("address of auditor node")
                    .short("-a")
                    .long("--auditor")
                    .required(true)
                    .takes_value(true),
            )
            .get_matches();

        let peer_id = matches.value_of("peer_id").expect("Set peer ID");
        let public_ip = matches.value_of("public_ip").expect("Set public IP");

        let mode;
        let mut asset_leger = AssetLedger::new(&peer_id);
        let chain_addr = Chain::new("Legit").start();
        let tampered_chain_addr = Chain::new("Tampered").start();
        let network_addr = Network::default().start();

        let tampered_asset_addr = AssetLedger::new("tampered_asset_addr").start();

        let init_network_size: usize = matches
            .value_of("network_size")
            .expect("set network_size")
            .parse()
            .unwrap();

        let auditor_node = matches
            .value_of("auditor")
            .expect("Set auditor node")
            .trim();

        match matches
            .value_of("mode")
            .expect("Set mode")
            .trim()
            .to_lowercase()
            .as_ref()
        {
            "auditor" => {
                mode = Mode::Auditor;
                asset_leger = AssetLedger::generate(&peer_id);
            }
            "victim" => mode = Mode::Victim(false),

            "attacker" => {
                mode = Mode::Attacker(false)
                //           tampered_asset_addr = Some(AssetLedger::default().start());
            }
            "normal" => {
                mode = Mode::Normal;
            }
            _ => panic!("Enter valid peer mode"),
        };

        let mode_addr = ModeActor::new(mode).start();
        Config {
            peer_id: peer_id.into(),
            mode_addr,
            //       tampered_asset_addr,
            asset_addr: asset_leger.start(),
            tampered_chain_addr,
            tampered_asset_addr,
            chain_addr,
            network_addr,
            init_network_size,
            auditor_node: auditor_node.into(),
            public_ip: public_ip.into(),
        }
    }
}

//impl Actor for Config {
//    type Context = Context<Self>;
//}
//
//#[derive(Message)]
//#[rtype(result = "()")]
//pub struct Sync;
//
//impl Handler<Sync> for Config {
//    type Result = ();
//    fn handle(&mut self, _msg: Sync, ctx: &mut Self::Context) -> Self::Result {
impl Config {
    pub async fn sync(&self) {
        use crate::chain::{DumpLedger as ChainDump, ReplaceChain};
        use crate::discovery::{DumpPeer, ReplacePeerLedger};
        use actix::clock::delay_for;
        use std::time::Duration;
        let duration = Duration::from_millis(1000);

        loop {
            let client = Client::default();
            let peers = self.network_addr.send(DumpPeer).await.unwrap();

            for peer in peers.iter() {
                delay_for(duration).await;
                //    let chain = client.get_chain(&self, &peer.id).await;
                //    let current_chain = self.chain_addr.send(ChainDump).await.unwrap();
                //    if current_chain.len() < chain.len() {
                //        self.chain_addr.send(ReplaceChain(chain)).await;
                //        client.get_peer_assets(&self, peer).await;
                //    }

                let peers_upadate = client.peer_dump(&self).await;
                if peers.len() < peers_upadate.len() {
                    self.debug("Refreshing peer ledger");
                    self.network_addr
                        .send(ReplacePeerLedger(peers_upadate))
                        .await
                        .unwrap();
                    self.debug("Refreshing asset ledger");
                    client.get_peer_assets(&self, peer).await;
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct ModeActor {
    pub mode: Mode,
}

impl ModeActor {
    pub fn new(mode: Mode) -> Self {
        ModeActor { mode }
    }
}

impl Actor for ModeActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SetMode(pub Mode);

#[derive(Message)]
#[rtype(result = "Mode")]
pub struct GetMode;

impl Handler<GetMode> for ModeActor {
    type Result = MessageResult<GetMode>;
    fn handle(&mut self, _msg: GetMode, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.mode.clone())
    }
}

impl Handler<SetMode> for ModeActor {
    type Result = MessageResult<SetMode>;
    fn handle(&mut self, msg: SetMode, _ctx: &mut Self::Context) -> Self::Result {
        self.mode = msg.0;
        MessageResult(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[actix_rt::test]
    async fn mode_actor_works() {
        let mode = ModeActor::new(Mode::Auditor).start();

        assert_eq!(Mode::Auditor, mode.send(GetMode).await.unwrap());
        mode.send(SetMode(Mode::Attacker(true))).await.unwrap();

        assert_eq!(Mode::Attacker(true), mode.send(GetMode).await.unwrap());
    }
}
