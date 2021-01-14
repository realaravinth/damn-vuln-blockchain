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
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/realaravinth/damn-vuln-blockchain/master/assets/block.svg"
)]
//!### Usage:
//!`Damn Vulnerable Blockchain` comes with a peer implementation called
//!`dwb`. `dwb` supports three modes:
//!
//!| Mode     | Function                                                                                     |
//!| -------- | -------------------------------------------------------------------------------------------- |
//!| Attacker | Configured to fork the blockchain and perform a double spend. See                            |
//!| Auditor  | This is a special peer that acts as the discovery node and mint. It should be spawned first. |
//!| Victim   | This peer will be configured to take itself down when an attack command is issued.           |
//!| Normal   | No special abilities, just an other node in the network                                      |
//!
//!
//!## `dwb` usage:
//!
//!```
//!Damn Vulnerable Blockchain 0.1
//!Aravinth Manivannan <realaravinth@batsense.net>
//!A bloody vulnerable blockchain implementation
//!
//!USAGE:
//!    dwb --auditor <auditor> --mode <mode> --network-size <network_size> --name <peer_id> --public-ip <public_ip>
//!
//!FLAGS:
//!    -h, --help       Prints help information
//!    -V, --version    Prints version information
//!
//!OPTIONS:
//!    -a, --auditor <auditor>              address of auditor node
//!    -m, --mode <mode>                    available modes:
//!                                         	auditor
//!                                         	normal
//!                                         	attacker
//!                                         	victim
//!    -s, --network-size <network_size>    set intial network size
//!    -n, --name <peer_id>                 set peer name
//!    -i, --public-ip <public_ip>          set public IP
//!```
//!
//!## Spinning up a cluster:
//!
//!The easiest way to spin up a cluster locally is using `./network.sh`.
//!
//!```
//!USAGE:
//!  ./network.sh
//!  launch   launches network
//!  kill     kills network
//!```
use actix_web::{error::InternalError, http::StatusCode, web::JsonConfig};

mod routes;
#[cfg(test)]
mod tests;

#[actix_web::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> std::io::Result<()> {
    use actix_web::{
        middleware::{normalize, Compress, Logger, NormalizePath},
        App, HttpServer,
    };
    use damn_vuln_blockchain::Client;
    use damn_vuln_blockchain::Config;

    pretty_env_logger::init();

    let config = Config::new();

    config.bootstrap().await;
    let ip_addr = config.public_ip.clone();

    HttpServer::new(move || {
        let log = &format!(
            "[{}]{}",
            &config.peer_id, "%a %r %s %b %{Referer}i %{User-Agent}i %T"
        );
        App::new()
            .configure(routes::services)
            .data(config.clone())
            .data(Client::default())
            .app_data(get_json_err())
            .wrap(Logger::new(log))
            .wrap(Compress::default())
            .wrap(NormalizePath::new(normalize::TrailingSlash::Trim))
    })
    .bind(ip_addr)?
    .run()
    .await
}

#[cfg(not(tarpaulin_include))]
fn get_json_err() -> JsonConfig {
    JsonConfig::default().error_handler(|err, _| {
        //debug!("JSON deserialization error: {:?}", &err);
        InternalError::new(err, StatusCode::BAD_REQUEST).into()
    })
}
