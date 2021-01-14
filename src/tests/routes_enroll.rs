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

    use actix_web::test;

    use damn_vuln_blockchain::asset::Asset;
    use damn_vuln_blockchain::config::{GetMode, Mode};
    use damn_vuln_blockchain::payload::Peer;
    use damn_vuln_blockchain::Config;

    use crate::routes::tests::{make_get_request, make_post_request};
    use damn_vuln_blockchain::helpers::*;

    pub async fn prepare_default_stake(config: &Config) -> Vec<String> {
        let mut default_stake_id: Vec<String> = Vec::new();
        let assets_for_me = get_my_assets(&config).await;
        assets_for_me.iter().for_each(|asset| {
            default_stake_id.push(asset.get_hash().to_owned());
        });

        default_stake_id
    }

    fn get_stake_payload(block_id: usize) -> Option<String> {
        use damn_vuln_blockchain::payload::GetStake as PayloadGetStake;

        let payload = serde_json::to_string(&PayloadGetStake { block_id }).unwrap();
        Some(payload)
    }

    #[actix_rt::test]
    async fn get_stake_route_works() {
        use damn_vuln_blockchain::asset::Stake;

        let config = init_network(Mode::Auditor).await;

        let default_stake_id = prepare_default_stake(&config).await;

        let resp = make_post_request(&config, get_stake_payload(5), "/stake").await;

        assert!(resp.status().is_success(), "get  stake is 200");
        let stake: Stake = test::read_body_json(resp).await;
        assert_eq!(stake.block_id, 5);
        assert_eq!(stake.stake, default_stake_id);
    }

    #[actix_rt::test]
    async fn victim_get_stake_route_works() {
        use damn_vuln_blockchain::asset::Stake;

        let config = init_network(Mode::Attacker(true)).await;
        let default_stake_id = prepare_default_stake(&config).await;

        // testing get stake where victim = true
        // should return empty_stake
        let resp = make_post_request(&config, get_stake_payload(5), "/stake").await;
        assert!(resp.status().is_success(), "get  stake is 200");
        let stake: Stake = test::read_body_json(resp).await;
        let empty_stake: Vec<String> = Vec::default();
        assert_eq!(stake.block_id, 5);
        assert_eq!(stake.stake, empty_stake);

        // testing get stake where attacker = false
        // should return full stake
        let config = init_network(Mode::Attacker(false)).await;
        let resp = make_post_request(&config, get_stake_payload(5), "/stake").await;
        assert!(resp.status().is_success(), "get  stake is 200");
        let stake: Stake = test::read_body_json(resp).await;
        assert_eq!(stake.block_id, 5);
        assert_eq!(stake.stake, default_stake_id);
    }

    #[actix_rt::test]
    async fn attacker_get_stake_route_works() {
        use damn_vuln_blockchain::asset::Stake;

        let config = init_network(Mode::Attacker(false)).await;
        let default_stake_id = prepare_default_stake(&config).await;

        // testing get stake where attacker = false
        // should return empty_stake
        let resp = make_post_request(&config, get_stake_payload(5), "/stake").await;
        assert!(resp.status().is_success(), "get  stake is 200");
        let stake: Stake = test::read_body_json(resp).await;
        let empty_stake: Vec<String> = Vec::default();
        assert_eq!(stake.block_id, 5);
        assert_eq!(stake.stake, empty_stake);

        // testing get stake where attacker = true
        // should return full stake
        let config = init_network(Mode::Attacker(true)).await;
        let resp = make_post_request(&config, get_stake_payload(5), "/stake").await;
        assert!(resp.status().is_success(), "get  stake is 200");
        let stake: Stake = test::read_body_json(resp).await;
        assert_eq!(stake.block_id, 5);
        assert_eq!(stake.stake, default_stake_id);
    }

    #[actix_rt::test]
    async fn dump_and_enroll_work() {
        let peer = Peer {
            id: "testing".into(),
            ip: "yolo".into(),
        };
        let payload = serde_json::to_string(&peer).unwrap();

        let config = init_network(Mode::Auditor).await;
        // testing peer enrollemnt

        let resp = make_post_request(&config, Some(payload), "/peers/enroll").await;
        assert!(resp.status().is_success(), "peer enrollment is 200");

        // testing peer dump by getting the dump and comparing it against
        // the peer that was enrolled in the previous test

        let resp = make_get_request(&config, "/peers/all").await;
        assert!(resp.status().is_success(), "peer dump is 200");
        let mut json_resp: Vec<Peer> = test::read_body_json(resp).await;
        assert_eq!(json_resp.pop().unwrap().ip, peer.ip, "peer dump works");

        // testing if the assets have been assigned to the newly enrolled peer
        let resp = make_get_request(&config, "/assets/all").await;
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
        let config = init_network(Mode::Attacker(false)).await;

        // testing toggle attack for attacker = false
        let resp = make_post_request(&config, None, "/attack").await;
        assert!(resp.status().is_success(), "set attack is 200");
        assert_eq!(
            config.mode_addr.send(GetMode).await.unwrap(),
            Mode::Attacker(true),
        );
    }

    #[actix_rt::test]
    async fn set_attack_works_attacker_true() {
        let config = init_network(Mode::Attacker(true)).await;

        // testing toggle attack for attacker = false
        let resp = make_post_request(&config, None, "/attack").await;
        assert!(resp.status().is_success(), "set attack is 200");
        assert_eq!(
            config.mode_addr.send(GetMode).await.unwrap(),
            Mode::Attacker(false),
        );
    }
    #[actix_rt::test]
    async fn set_attack_works_victim_true() {
        let config = init_network(Mode::Victim(true)).await;

        let resp = make_post_request(&config, None, "/attack").await;
        assert!(resp.status().is_success(), "set attack is 200");
        assert_eq!(
            config.mode_addr.send(GetMode).await.unwrap(),
            Mode::Victim(false),
        );
    }
    #[actix_rt::test]
    async fn set_attack_works_victim_false() {
        let config = init_network(Mode::Victim(false)).await;

        // testing toggle attack for attacker = false
        let resp = make_post_request(&config, None, "/attack").await;
        assert!(resp.status().is_success(), "set attack is 200");
        assert_eq!(
            config.mode_addr.send(GetMode).await.unwrap(),
            Mode::Victim(true),
        );
    }
}
