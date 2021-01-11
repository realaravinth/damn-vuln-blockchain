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

use damn_vuln_blockchain::asset::AssetLedger;
use damn_vuln_blockchain::chain::Chain;
use damn_vuln_blockchain::discovery::Network;

mod routes;
mod test;

#[derive(Clone)]
pub struct Config {
    pub mode: Mode,
    pub peer_id: String,
    pub port: usize,
    pub asset_addr: Addr<AssetLedger>,
    pub chain_addr: Addr<Chain>,
    pub tampered_chain_addr: Option<Addr<Chain>>,
    pub network_addr: Addr<Network>,
    pub init_network_size: usize,
}

#[derive(Clone)]
pub enum Mode {
    Auditor,
    Attacker,
    Victim,
}

#[actix_web::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> std::io::Result<()> {
    use actix_web::{
        middleware::{normalize, Compress, Logger, NormalizePath},
        App, HttpServer,
    };
    pretty_env_logger::init();
    let config = cli();
    let ip_addr = format!("0.0.0.0:{}", config.port);

    HttpServer::new(move || {
        App::new()
            .configure(routes::services)
            .data(config.clone())
            .app_data(get_json_err())
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(NormalizePath::new(normalize::TrailingSlash::Trim))
    })
    .bind(ip_addr)?
    .run()
    .await
}

use actix_web::{error::InternalError, http::StatusCode, web::JsonConfig};
#[cfg(not(tarpaulin_include))]
fn get_json_err() -> JsonConfig {
    JsonConfig::default().error_handler(|err, _| {
        //debug!("JSON deserialization error: {:?}", &err);
        InternalError::new(err, StatusCode::BAD_REQUEST).into()
    })
}

#[cfg(not(tarpaulin_include))]
fn cli() -> Config {
    use clap::{App, Arg};
    let matches = App::new("Damn Vulnerable Blockchain")
        .version("0.1")
        .author("Aravinth Manivannan <realaravinth@batsense.net>")
        .about("A bloody vulnerable blockchain implementation")
        .arg(
            Arg::with_name("port")
                .help("set port to listen on")
                .short("-p")
                .long("--port")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("peer_id")
                .help("set peer ID")
                .short("-i")
                .long("--id")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("network_size")
                .help("set intial network size")
                .short("-n")
                .long("--network-size")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("mode")
                .help("available modes:\n\tauditor\n\tattacker\n\tvictim ")
                .short("-m")
                .long("--mode")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("discovery")
                .help("address of discovery node")
                .short("-d")
                .long("--discovery")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let peer_id = matches.value_of("peer_id").expect("Set peer ID");

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
        _ => panic!("Enter valid peer mode"),
    };

    let port: usize = matches.value_of("port").unwrap().parse().unwrap();

    Config {
        peer_id: peer_id.into(),
        port,
        mode,
        //       tampered_asset_addr,
        asset_addr: asset_leger.start(),
        tampered_chain_addr,
        chain_addr,
        network_addr,
        init_network_size,
    }
}
