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

    use crate::tests::api_tests::tests::stake_toggle_test;
    use crate::tests::tx::tests::tx_works;
    use damn_vuln_blockchain::helpers::*;
    use damn_vuln_blockchain::Client;
    use damn_vuln_blockchain::Config;

    async fn sync(config: Config) {
        config.sync().await;
        use actix::clock::delay_for;
        use damn_vuln_blockchain::chain::{DumpLedger as ChainDump, ReplaceChain};
        use damn_vuln_blockchain::discovery::{DumpPeer, ReplacePeerLedger};
        use std::time::Duration;
        let duration = Duration::from_millis(500);

        loop {
            let client = Client::default();
            let peers = config.network_addr.send(DumpPeer).await.unwrap();

            for peer in peers.iter() {
                delay_for(duration).await;
                let chain = client.get_chain(&config, &peer.ip).await;
                let current_chain = config.chain_addr.send(ChainDump).await.unwrap();
                if current_chain.len() < chain.len() {
                    config.chain_addr.send(ReplaceChain(chain)).await;
                    client.get_peer_assets(&config, peer).await;
                }

                let peers_upadate = client.peer_dump(&config).await;
                if peers.len() < peers_upadate.len() {
                    config
                        .network_addr
                        .send(ReplacePeerLedger(peers_upadate))
                        .await;
                    client.get_peer_assets(&config, peer).await;
                }
            }
        }
    }

    #[actix_rt::test]
    pub async fn race_cond_test() {
        let config = generate_test_config();
        actix::spawn(sync(config.clone()));

        let client = Client::default();
        non_register_bootstrap(&config, &client).await;

        tx_works(&config, &client).await;
        use actix::clock::delay_for;
        use std::time::Duration;
        let duration = Duration::from_millis(5000);
        delay_for(duration).await;

        // blocking on sync
        stake_toggle_test(&config, &client).await;
    }
}
