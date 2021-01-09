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

mod routes;

#[derive(Clone)]
struct Config {
    pub auditor: bool,
    pub peer_id: String,
    pub port: usize,
}

#[actix_web::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    let config = cli();
    let ip_addr = format!("0.0.0.0:{}", config.port);

    HttpServer::new(move || App::new().configure(routes::services).data(config.clone()))
        .bind(ip_addr)?
        .run()
        .await
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
            Arg::with_name("auditor")
                .help("runs peer in auditor mode")
                .short("-a")
                .long("--auditor")
                .required(false)
                .takes_value(false),
        )
        .get_matches();

    let peer_id = matches.value_of("peer_id").expect("Set peer ID");
    let auditor = matches.is_present("auditor");
    let port: usize = matches.value_of("port").unwrap().parse().unwrap();

    Config {
        auditor,
        peer_id: peer_id.into(),
        port,
    }
}
