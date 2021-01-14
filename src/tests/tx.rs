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

    use damn_vuln_blockchain::helpers::*;
    use damn_vuln_blockchain::payload::TxBuilder;
    use damn_vuln_blockchain::Client;
    use damn_vuln_blockchain::Config;

    pub async fn tx_works(config: &Config, client: &Client) {
        use damn_vuln_blockchain::asset::GetPeerAssets;

        use damn_vuln_blockchain::client::GetStake;

        // getting assets belonging to the seller peer
        let seller_peer_id = "victim.batsense.net";
        let seller_assets = config
            .asset_addr
            .send(GetPeerAssets(seller_peer_id.to_owned()))
            .await
            .unwrap();
        let asset_id = seller_assets.first().unwrap().get_hash();
        // preparing client msg
        let client_msg = TxBuilder::default()
            .asset_id(asset_id.into())
            .buyer_peer_id("attacker.batsense.net".into())
            .build()
            .unwrap();

        // sell request
        client
            .sell_asset(&config, &seller_peer_id, &client_msg)
            .await;

        // difficult to tell which peer will have the latest chain
        // so fetch both
        let mut attacker_chain = client.get_chain(&config, "attacker.batsense.net").await;
        let mut victim_chain = client.get_chain(&config, "victim.batsense.net").await;

        if attacker_chain.len() > victim_chain.len() {
            assert_eq!(
                attacker_chain.pop().unwrap().get_asset_id().unwrap(),
                asset_id
            );
        } else {
            assert_eq!(
                victim_chain.pop().unwrap().get_asset_id().unwrap(),
                asset_id
            );
        }

        //        assert_eq!(seller.block_id, block_id);
        //        assert_eq!(seller.seller, default_seller_id);
    }
}
