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

#[cfg(test)]
mod tests {

    use actix_web::{http::header, test, App};

    use damn_vuln_blockchain::asset::Asset;
    use damn_vuln_blockchain::config::{Mode, SetMode};
    use damn_vuln_blockchain::payload::Peer;

    use crate::routes::services;
    use crate::tests::helpers::generate_test_config;

    #[actix_rt::test]
    async fn get_stake_route_works() {
        use damn_vuln_blockchain::asset::{GetPeerAssets, GetStake, InitNetworkBuilder, Stake};
        use damn_vuln_blockchain::payload::GetStake as PayloadGetStake;

        let config = generate_test_config();
        config.mode_addr.send(SetMode(Mode::Auditor)).await.unwrap();

        let msg = InitNetworkBuilder::default()
            .network_size(config.init_network_size)
            .peer_id(config.peer_id.clone())
            .build()
            .unwrap();

        config.asset_addr.send(msg).await.unwrap();

        let assets_for_me = config
            .asset_addr
            .send(GetPeerAssets(config.peer_id.clone()))
            .await
            .unwrap();

        let mut app = test::init_service(App::new().configure(services).data(config.clone())).await;

        let mut default_stake_id: Vec<String> = Vec::new();
        assets_for_me.iter().for_each(|asset| {
            default_stake_id.push(asset.get_hash().to_owned());
        });

        // testing get stake
        let payload = serde_json::to_string(&PayloadGetStake { block_id: 5 }).unwrap();
        let req = test::TestRequest::post()
            .uri("/stake")
            .header(header::CONTENT_TYPE, "applicatin/json")
            .set_payload(payload)
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        println!("{:#?}", &resp);
        assert!(resp.status().is_success(), "get  stake is 200");
        let stake: Stake = test::read_body_json(resp).await;

        assert_eq!(stake.block_id, 5);
        assert_eq!(stake.stake, default_stake_id);
    }
    #[actix_rt::test]
    async fn attacker_get_stake_route_works() {
        use damn_vuln_blockchain::asset::{GetPeerAssets, GetStake, InitNetworkBuilder, Stake};
        use damn_vuln_blockchain::payload::GetStake as PayloadGetStake;

        let config = generate_test_config();
        config
            .mode_addr
            .send(SetMode(Mode::Attacker(false)))
            .await
            .unwrap();

        let msg = InitNetworkBuilder::default()
            .network_size(config.init_network_size)
            .peer_id(config.peer_id.clone())
            .build()
            .unwrap();

        config.asset_addr.send(msg).await.unwrap();

        let assets_for_me = config
            .asset_addr
            .send(GetPeerAssets(config.peer_id.clone()))
            .await
            .unwrap();

        let mut app = test::init_service(App::new().configure(services).data(config.clone())).await;

        let mut default_stake_id: Vec<String> = Vec::new();
        assets_for_me.iter().for_each(|asset| {
            default_stake_id.push(asset.get_hash().to_owned());
        });

        // testing get stake
        let payload = serde_json::to_string(&PayloadGetStake { block_id: 5 }).unwrap();
        let req = test::TestRequest::post()
            .uri("/stake")
            .header(header::CONTENT_TYPE, "applicatin/json")
            .set_payload(payload.clone())
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success(), "get  stake is 200");
        let stake: Stake = test::read_body_json(resp).await;

        let empty_stake: Vec<String> = Vec::default();
        assert_eq!(stake.block_id, 5);
        assert_eq!(stake.stake, empty_stake);

        config
            .mode_addr
            .send(SetMode(Mode::Attacker(true)))
            .await
            .unwrap();
        let req = test::TestRequest::post()
            .uri("/stake")
            .header(header::CONTENT_TYPE, "applicatin/json")
            .set_payload(payload.clone())
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success(), "get  stake is 200");
        let stake: Stake = test::read_body_json(resp).await;

        assert_eq!(stake.block_id, 5);
        assert_eq!(stake.stake, default_stake_id);
    }

    #[actix_rt::test]
    async fn dump_and_enroll_work() {
        let mut app = test::init_service(
            App::new()
                .configure(services)
                .data(generate_test_config().clone()),
        )
        .await;
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

        let network_size = generate_test_config().init_network_size;
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
                    "test.batsense.net",
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

    #[actix_rt::test]
    async fn set_attack_works() {
        use damn_vuln_blockchain::config::{GetMode, Mode};

        let config = generate_test_config();
        config
            .mode_addr
            .send(SetMode(Mode::Attacker(false)))
            .await
            .unwrap();

        let mut app = test::init_service(App::new().configure(services).data(config.clone())).await;

        // testing get stake
        let req = test::TestRequest::post().uri("/attack").to_request();

        let resp = test::call_service(&mut app, req).await;

        println!("{:#?}", &resp);
        assert!(resp.status().is_success(), "set attack is 200");
        assert_eq!(
            config.mode_addr.send(GetMode).await.unwrap(),
            Mode::Attacker(true),
        );
    }
}
