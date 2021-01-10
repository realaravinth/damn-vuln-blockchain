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

use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};

use crate::Config;
use damn_vuln_blockchain::logs::Peer;

//#[post("/assets/buy")]
//async fn asset_buy(data: web::Data<Config>) -> impl Responder {
//}

// peer enrollment
#[post("/peer/enroll")]
async fn peer_enroll(peer: web::Json<Peer>, data: web::Data<Config>) -> impl Responder {
    use damn_vuln_blockchain::asset::InitNetworkBuilder;
    use damn_vuln_blockchain::discovery::{AddPeer, GetCurrentSize};

    // peer enrollment should only happen when current_network_size < data.init_network_size
    let current_network_size = data.network_addr.send(GetCurrentSize).await.unwrap();
    if current_network_size < data.init_network_size {
        let asset_message = InitNetworkBuilder::default()
            .network_size(data.init_network_size)
            .peer_id(peer.id.clone())
            .build()
            .unwrap();

        data.network_addr
            .send(AddPeer(peer.into_inner()))
            .await
            .unwrap();

        data.asset_addr.send(asset_message).await.unwrap();
    }

    // TODO, must return error when current_network_size == data.init_network_size
    HttpResponse::Ok()
}

// peer enrollment
#[get("/peer/discover/all")]
async fn peer_dump(data: web::Data<Config>) -> impl Responder {
    use damn_vuln_blockchain::discovery::DumpPeer;
    let peer_data = data.network_addr.send(DumpPeer).await.unwrap();
    HttpResponse::Ok().json(peer_data)
}

// asset dump
#[get("/assets/all")]
async fn assets_dump(data: web::Data<Config>) -> impl Responder {
    use damn_vuln_blockchain::asset::DumpLedger;
    let assets = data.asset_addr.send(DumpLedger).await.unwrap();
    HttpResponse::Ok().json(assets)
}

//// buy asset
//#[post("/assets/buy")]
//async fn assets_coinage(data: web::Data<Config>) -> impl Responder {
//    let l
//}

pub fn services(cfg: &mut ServiceConfig) {
    cfg.service(peer_enroll);
    cfg.service(peer_dump);
    cfg.service(assets_dump);
}

#[cfg(test)]
mod tests {

    use actix::prelude::*;
    use actix_web::{http::header, test, App};

    use damn_vuln_blockchain::asset::{Asset, AssetLedger};
    use damn_vuln_blockchain::chain::Chain;
    use damn_vuln_blockchain::discovery::Network;

    use super::*;

    fn get_data() -> Config {
        let peer_id = "testnet";

        let mode = crate::Mode::Auditor;
        let asset_leger = AssetLedger::generate();
        let chain_addr = Chain::new("Legit").start();
        let tampered_chain_addr = None;
        let network_addr = Network::default().start();
        let init_network_size = 2;

        let port: usize = 8081; // dummy

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

    #[actix_rt::test]
    async fn dump_and_enroll_work() {
        let mut app =
            test::init_service(App::new().configure(services).data(get_data().clone())).await;
        let peer = Peer {
            id: "testing".into(),
            ip: "yolo".into(),
        };
        let payload = serde_json::to_string(&peer).unwrap();

        // testing peer enrollemnt
        let req = test::TestRequest::post()
            .uri("/peer/enroll")
            .header(header::CONTENT_TYPE, "applicatin/json")
            .set_payload(payload)
            .to_request();

        let mut resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success(), "peer enrollment is 200");

        // testing peer dump by getting the dump and comparing it against
        // the peer that was enrolled in the previous test
        let req = test::TestRequest::get()
            .uri("/peer/discover/all")
            .to_request();
        resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success(), "peer dump is 200");
        let mut json_resp: Vec<Peer> = test::read_body_json(resp).await;
        assert_eq!(json_resp.pop().unwrap().ip, peer.ip, "peer dump works");

        // testing if the assets have been assigned to the newly enrolled peer
        let req = test::TestRequest::get().uri("/assets/all").to_request();
        resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success(), "peer dump is 200");
        let json_resp: Vec<Asset> = test::read_body_json(resp).await;

        let network_size = get_data().init_network_size;
        //        assert_eq!(json_resp.pop().unwrap().ip, peer.ip, "peer dump works");
        //
        // total number of assets:
        let length = json_resp.len();
        // total number of assets that should be assigned to a new peer
        let assets_per_peer = length / network_size;

        let mut asset_ledger_per_peer_state = 0;

        // checking if ownsership is alright
        for i in json_resp.iter() {
            if i.get_owner().is_some() {
                // ownership is verified here if ownder != "testing", then the
                // below statement should panic
                assert_eq!(
                    i.get_owner().as_ref().unwrap(),
                    "testing",
                    "asset ownder rightly assigned"
                );
                // counting asset to peer "testing"(only testing, see above comment)
                asset_ledger_per_peer_state += 1;
            }
        }

        // checking assets for over/under assignment to new peers
        assert_eq!(
            assets_per_peer, asset_ledger_per_peer_state,
            "assets per peer satisfied, no over allocation, no under allocation"
        );
    }
}
