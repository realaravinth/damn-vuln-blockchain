use log::debug;
use pretty_env_logger;

mod client;

#[actix_web::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init;

    use clap::{App, Arg, SubCommand};
    let matches = App::new("Damn Vulnerable Blockchain Client")
        .version("0.1")
        .author("Aravinth Manivannan <realaravinth@batsense.net>")
        .about("Client for Damn Vulnerable Blockchain")
        .subcommand(
            App::new("peers")
                .about("Get peers")
                .arg(
                    Arg::with_name("auditor_ip")
                        .help("set auditor node's IP")
                        .short("-a")
                        .long("--auditor-ip")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("public_id")
                        .help("set peer ID")
                        .short("-i")
                        .long("--public-id")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            App::new("sell")
                .about("Sell asset")
                .arg(
                    Arg::with_name("public_id")
                        .help("set owner peer ID")
                        .short("-i")
                        .long("--public-id")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("asset_id")
                        .help("set asset ID")
                        .long("--asset-id")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("auditor_ip")
                        .help("set auditor node's IP")
                        .short("-a")
                        .long("--auditor-ip")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("buyer_id")
                        .help("set buyer ID")
                        .short("-b")
                        .long("--buyer-id")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            App::new("attack")
                .about("Set attack")
                .arg(
                    Arg::with_name("auditor_ip")
                        .help("set auditor node's IP")
                        .short("-a")
                        .long("--auditor-ip")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("public_id")
                        .help("set peer ID")
                        .short("-i")
                        .long("--public-id")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            App::new("fork")
                .about("Fork chain")
                .arg(
                    Arg::with_name("auditor_ip")
                        .help("set auditor node's IP")
                        .short("-a")
                        .long("--auditor-ip")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("public_id")
                        .help("set peer ID")
                        .short("-i")
                        .long("--public-id")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            App::new("dump")
                .about("Get chain")
                .arg(
                    Arg::with_name("auditor_ip")
                        .help("set auditor node's IP")
                        .short("-a")
                        .long("--auditor-ip")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("peer_id")
                        .help("set peer ID")
                        .short("-p")
                        .long("--peer-id")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("item")
                        .help("set item name(chain/asset)")
                        .short("-t")
                        .long("--item")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("attack") => {
            let id = matches.value_of("public_id").unwrap();
            let auditor = matches.value_of("auditor_ip").unwrap();
            attack(id, auditor).await;
        }
        Some("dump") => (),
        Some("fork") => (),
        Some("peers") => (),
        Some("sell") => (),
        None => (),
        _ => unreachable!(),
    }

    Ok(())
}

async fn attack(id: &str, auditor: &str) {
    use client::Client;
    let client = Client::default();
    let target = client
        .discovery(&id, &auditor)
        .await
        .expect("Peer not found");
    client.set_attack(target).await;
}
