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

    use damn_vuln_blockchain::asset::DumpLedger;
    use damn_vuln_blockchain::discovery::GetPeer;
    use damn_vuln_blockchain::Client;

    use crate::tests::helpers::*;

    // testing api client wrappers from
    // damn_vuln_blockchain::api::client::Client
    // it's kinda nasty as it's both integration
    // and whitebox at the same time but it works
    #[actix_rt::test]
    async fn dump_and_enroll_work() {
        let config = generate_test_config();
        let mut client = Client::default();

        client.peer_enroll(&config).await;
        client.peer_discovery(&config).await;

        // checking if peer enrollment works
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

        // testing get_all_assets
        client.get_all_assets(&config).await;
        // getting dump from internal actor
        let dump = config.asset_addr.send(DumpLedger).await.unwrap();

        // calculating assets per peer
        let length = dump.len();
        let assets_per_peer = length / config.init_network_size;

        // flags and stuff for tests
        let mut asset_ledger_per_peer_state = 0;
        let mut this_peer_got_assets_flag = false;

        // iterating through dump and checking if
        // this peer has received its rightful share
        // of assets
        for i in dump.iter() {
            if i.get_owner().is_some() {
                // ownership is verified here if ownder != "me", then the
                // below statement should panic
                if i.get_owner().as_ref().unwrap() == &config.peer_id {
                    this_peer_got_assets_flag = true;
                    asset_ledger_per_peer_state += 1;
                }
            }
        }
        assert!(this_peer_got_assets_flag, "get_all_assets works");
        assert_eq!(
            assets_per_peer, asset_ledger_per_peer_state,
            "assets per peer satisfied, no over allocation, no under allocation"
        );
    }

    #[actix_rt::test]
    async fn get_stake_victim_peer() {
        use damn_vuln_blockchain::asset::{GetPeerAssets, Stake};

        use damn_vuln_blockchain::client::GetStake;

        let config = generate_test_config();
        config.bootstrap().await;

        // testing victim client
        let stake_peer_id = "victim.batsense.net";
        let assets_for_me = config
            .asset_addr
            .send(GetPeerAssets(stake_peer_id.to_owned()))
            .await
            .unwrap();

        let mut default_stake_id: Vec<String> = Vec::new();
        assets_for_me.iter().for_each(|asset| {
            default_stake_id.push(asset.get_hash().to_owned());
        });

        let client = Client::default();
        let block_id = 9999;

        let client_msg = GetStake {
            block_id,
            peer_id: stake_peer_id.into(),
        };

        let stake: Stake = client.get_stake(client_msg, &config).await;
        assert_eq!(stake.block_id, block_id);
        assert_eq!(stake.stake, default_stake_id);
    }

    #[actix_rt::test]
    async fn get_stake_attacker_peer() {
        use damn_vuln_blockchain::asset::{GetPeerAssets, Stake};
        use damn_vuln_blockchain::client::GetStake;

        let config = generate_test_config();
        config.bootstrap().await;

        // testing attakcing peer when Mode::Attacker(false)
        let stake_peer_id = "attacker.batsense.net";
        let assets_for_me = config
            .asset_addr
            .send(GetPeerAssets(stake_peer_id.to_owned()))
            .await
            .unwrap();

        let mut default_stake_id: Vec<String> = Vec::new();
        assets_for_me.iter().for_each(|asset| {
            default_stake_id.push(asset.get_hash().to_owned());
        });

        let client = Client::default();
        let block_id = 9999;

        let client_msg = GetStake {
            block_id,
            peer_id: stake_peer_id.into(),
        };

        let stake: Stake = client.get_stake(client_msg, &config).await;
        assert_eq!(stake.block_id, block_id);
        assert_eq!(stake.stake, Stake::default().stake);

        // testing attakcing peer when Mode::Attacker(true)

        client.set_attack(&config).await;

        let block_id = 3;
        let client_msg = GetStake {
            block_id,
            peer_id: stake_peer_id.into(),
        };
        let stake: Stake = client.get_stake(client_msg, &config).await;
        assert_eq!(stake.block_id, block_id);
        assert_eq!(stake.stake, default_stake_id);
    }
}
