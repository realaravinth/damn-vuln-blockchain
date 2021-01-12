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
pub mod routes_enroll;

#[cfg(test)]
pub mod api_tests;

#[cfg(test)]
pub mod helpers {
    use actix::prelude::*;
    use damn_vuln_blockchain::asset::AssetLedger;
    use damn_vuln_blockchain::chain::Chain;
    use damn_vuln_blockchain::config::{Config, Mode};
    use damn_vuln_blockchain::discovery::Network;

    #[cfg(test)]
    pub fn generate_test_config() -> Config {
        let peer_id = "test.bastsense.net".into();
        let public_ip = "localhost:7003".into();

        let mode = Mode::Normal;
        let chain_addr = Chain::new("Legit").start();
        let tampered_chain_addr = None;
        let network_addr = Network::default().start();

        let init_network_size: usize = 3;
        let auditor_node = "localhost:7000".into();

        Config {
            peer_id,
            mode,
            asset_addr: AssetLedger::default().start(),
            tampered_chain_addr,
            chain_addr,
            network_addr,
            init_network_size,
            auditor_node,
            public_ip,
        }
    }
}
