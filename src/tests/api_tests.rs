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
pub mod tests {

    use damn_vuln_blockchain::asset::DumpLedger;
    use damn_vuln_blockchain::utils::*;
    use damn_vuln_blockchain::Client;
    use damn_vuln_blockchain::Config;

    use damn_vuln_blockchain::helpers::*;

    // testing api client wrappers from
    // damn_vuln_blockchain::api::client::Client
    // it's kinda nasty as it's both integration
    // and whitebox at the same time but it works
    #[actix_rt::test]
    async fn dump_and_enroll_work() {
        let config = generate_test_config();
        let client = Client::default();

        client.peer_enroll(&config).await;
        client.peer_discovery(&config).await;

        // checking if peer enrollment works
        assert_eq!(
            get_peer(&config, "test.bastsense.net").await.ip,
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

    // race condition, toggles attack mode. Can mess up stake and sync
    pub async fn stake_toggle_test(config: &Config, client: &Client) {
        use damn_vuln_blockchain::asset::Stake;
        use damn_vuln_blockchain::client::GetStake;

        // let config = generate_test_config();
        // let client = Client::default();

        // non_register_bootstrap(&config, &client).await;

        // testing attakcing peer when Mode::Attacker(false)
        let attacker_id = "attacker.batsense.net";
        let attacker_default_stake = get_default_stake(&config, attacker_id).await;

        let victim_id = "victim.batsense.net";

        let victim_default_stake = get_default_stake(&config, victim_id).await;

        let block_id = 9999;
        let mut attacker_client_msg = GetStake {
            block_id,
            peer_id: attacker_id.into(),
        };
        let mut victim_client_msg = GetStake {
            block_id,
            peer_id: victim_id.into(),
        };

        let stake: Stake = client.get_stake(victim_client_msg.clone(), &config).await;
        assert_eq!(stake.block_id, block_id);
        assert_eq!(stake.stake, victim_default_stake);
        let stake: Stake = client.get_stake(attacker_client_msg.clone(), &config).await;
        assert_eq!(stake.block_id, block_id);
        assert_eq!(stake.stake, Stake::default().stake);

        // testing stake after toggling peers
        client.set_attack(&config).await;

        let block_id = 3;
        attacker_client_msg.block_id = block_id;
        victim_client_msg.block_id = block_id;
        let stake: Stake = client.get_stake(attacker_client_msg, &config).await;
        assert_eq!(stake.block_id, block_id);
        assert_eq!(stake.stake, attacker_default_stake);
        let stake: Stake = client.get_stake(victim_client_msg, &config).await;
        assert_eq!(stake.block_id, block_id);
        assert_eq!(stake.stake, Stake::default().stake);
    }
}
