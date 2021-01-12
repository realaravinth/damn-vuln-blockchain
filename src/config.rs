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
use log::info;

use crate::api::Client;
use crate::asset::AssetLedger;
use crate::chain::Chain;
use crate::discovery::Network;

#[derive(Clone)]
pub struct Config {
    pub mode: Mode,
    pub peer_id: String,
    pub public_ip: String,
    pub auditor_node: String,
    pub asset_addr: Addr<AssetLedger>,
    pub chain_addr: Addr<Chain>,
    pub tampered_chain_addr: Option<Addr<Chain>>,
    pub network_addr: Addr<Network>,
    pub init_network_size: usize,
}

#[derive(Clone, PartialEq)]
pub enum Mode {
    Auditor,
    Attacker,
    Victim,
    Normal,
}

impl Config {
    #[cfg(not(tarpaulin_include))]
    pub fn new() -> Self {
        Config::cli()
    }

    pub async fn bootstrap(&self) {
        if self.mode != Mode::Auditor {
            info!("Bootstrapping node");
            let mut client = Client::default();
            info!("Enrolling peer");
            client.peer_enroll(&self).await;
            info!("Discovering peers in network");
            client.peer_discovery(&self).await;
            info!("Bootstrapping assets");
            client.get_all_assets(&self).await;
        }
    }

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
        let mut asset_leger = AssetLedger::default();
        let chain_addr = Chain::new("Legit").start();
        let tampered_chain_addr = None;
        let network_addr = Network::default().start();

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
                asset_leger = AssetLedger::generate();
            }
            "victim" => mode = Mode::Victim,

            "attacker" => {
                mode = Mode::Attacker;
                //           tampered_asset_addr = Some(AssetLedger::default().start());
            }
            "normal" => {
                mode = Mode::Normal;
            }
            _ => panic!("Enter valid peer mode"),
        };

        Config {
            peer_id: peer_id.into(),
            mode,
            //       tampered_asset_addr,
            asset_addr: asset_leger.start(),
            tampered_chain_addr,
            chain_addr,
            network_addr,
            init_network_size,
            auditor_node: auditor_node.into(),
            public_ip: public_ip.into(),
        }
    }
}