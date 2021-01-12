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

    use damn_vuln_blockchain::api::Client;
    use damn_vuln_blockchain::discovery::GetPeer;

    use crate::tests::helpers::*;

    #[actix_rt::test]
    async fn dump_and_enroll_work() {
        let config = generate_test_config();
        let mut client = Client::default();

        client.peer_enroll(&config).await;
        client.peer_discovery(&config).await;

        assert_eq!(
            config
                .network_addr
                .send(GetPeer("test.bastsense.net".into()))
                .await
                .unwrap()
                .unwrap()
                .ip,
            "localhost:7003",
            "peer_enroll and peer_discovery works"
        );

        //        resp = test::call_service(&mut app, req).await;
        //        assert!(resp.status().is_success(), "peer dump is 200");
        //        let mut json_resp: Vec<Peer> = test::read_body_json(resp).await;
        //        assert_eq!(json_resp.pop().unwrap().ip, peer.ip, "peer dump works");
        //
        //        // testing if the assets have been assigned to the newly enrolled peer
        //        let req = test::TestRequest::get().uri("/assets/all").to_request();
        //        resp = test::call_service(&mut app, req).await;
        //        assert!(resp.status().is_success(), "peer dump is 200");
        //        let json_resp: Vec<Asset> = test::read_body_json(resp).await;
        //
        //        let network_size = get_data().init_network_size;
        //        // total number of assets:
        //        let length = json_resp.len();
        //        // total number of assets that should be assigned to a new peer
        //        let assets_per_peer = length / network_size;
        //
        //        let mut asset_ledger_per_peer_state = 0;
        //
        //        // checking if ownsership is alright
        //        for i in json_resp.iter() {
        //            if i.get_owner().is_some() {
        //                // ownership is verified here if ownder != "testing", then the
        //                // below statement should panic
        //                assert_eq!(
        //                    i.get_owner().as_ref().unwrap(),
        //                    "testing",
        //                    "asset ownder rightly assigned"
        //                );
        //                // counting asset to peer "testing"(only testing, see above comment)
        //                asset_ledger_per_peer_state += 1;
        //            }
        //        }
        //
        //        // checking assets for over/under assignment to new peers
        //        assert_eq!(
        //            assets_per_peer, asset_ledger_per_peer_state,
        //            "assets per peer satisfied, no over allocation, no under allocation"
        //        );
    }
}
