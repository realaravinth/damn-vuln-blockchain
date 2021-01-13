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
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/realaravinth/damn-vuln-blockchain/master/assets/block.svg"
)]
//! This is a test blockchain that I build for fun and as the name
//! suggests, **it is bloody vulnerable.**
//!
//! This library serves as its building block.
//!
//! ## Create a new blockchain
//! ```rust
//! use damn_vuln_blockchain::{ asset::AssetLedger, block::{BlockBuilder, Block}, chain::Chain};
//!
//! fn main() {
//!        let chain = Chain::new("My chain"); // crate cahin
//!   }
//! ```
//!
//! ## Create a block
//! ```rust
//! use damn_vuln_blockchain::{ asset::AssetLedger, block::{BlockBuilder, Block}, chain::Chain};
//!
//!
//! fn main() {
//!        let chain = Chain::new("My chain"); // create blockchain
//!        let mut assets = AssetLedger::generate(); // generate some assets
//!
//!        let asset = assets.assets.pop().unwrap();
//!
//!        // get the last block of a chain
//!        let prev = chain.get_last_block();
//!
//!        let block = BlockBuilder::default()
//!            .set_tx("Me")
//!            .set_rx("You")
//!            .set_prev(&prev)
//!            .set_asset_id(&asset)
//!            .set_validator("Me")
//!            .build();
//!
//!        assert!(!block.is_genesis());
//!        assert_eq!(block.get_tx().unwrap(), "Me");
//!        assert_eq!(block.get_rx().unwrap(), "You");
//! }
//! ```

pub mod asset;
pub mod block;
pub mod chain;
pub mod client;
pub mod config;
pub mod discovery;
pub mod error;
pub mod logs;
pub mod payload;
mod utils;

pub use client::Client;
pub use config::Config;
