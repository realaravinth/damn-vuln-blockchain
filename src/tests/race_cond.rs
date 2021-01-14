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

    #[actix_rt::test]
    pub async fn race_cond_test() {
        let config = generate_test_config();

        let client = Client::default();
        non_register_bootstrap(&config, &client).await;

        tx_works(&config, &client).await;
        // blocking on sync
        // stake_toggle_test(&config, &client).await;
    }
}
