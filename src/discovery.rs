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

//! Peer management service for synchronising and discovery.
//! The easiest way to interact with the peer management service
//! is with the [Network] actor.
//!
//! # [Network] actor supports the following messages:
//! - [AddPeer]: Add a peer to the network
//! - [DumpPeer]: Get a dump of all peers in the network
//! - [GetPeer]: Get ingo on a specific peer
//! - [ReplacePeerLedger]: Replace peer ledger

use actix::prelude::*;
use serde::{Deserialize, Serialize};

use crate::payload::Peer;

#[derive(Deserialize, Default, Serialize, Clone, Debug)]
pub struct Network {
    peer: Vec<Peer>,
}

impl Actor for Network {
    type Context = Context<Self>;
}

/// Add peer
#[derive(Message)]
#[rtype(result = "()")]
pub struct AddPeer(pub Peer);

/// Dump all peers
#[derive(Message)]
#[rtype(result = "Vec<Peer>")]
pub struct DumpPeer;

/// Get peer of ID
#[derive(Message)]
#[rtype(result = "Option<Peer>")]
pub struct GetPeer(pub String);

/// Get current network size
#[derive(Message)]
#[rtype(result = "usize")]
pub struct GetCurrentSize;

/// Get peer of ID
#[derive(Message)]
#[rtype(result = "()")]
pub struct ReplacePeerLedger(pub Vec<Peer>);

impl Network {
    fn get_peer_index(&self, id: &str) -> Option<usize> {
        let mut target: Option<usize> = None;
        for (index, peer) in self.peer.iter().enumerate() {
            if peer.id == id {
                target = Some(index);
                break;
            }
        }
        target
    }
}

impl Handler<AddPeer> for Network {
    type Result = MessageResult<AddPeer>;

    fn handle(&mut self, msg: AddPeer, _ctx: &mut Self::Context) -> Self::Result {
        if let None = self.get_peer_index(&msg.0.id) {
            self.peer.push(msg.0);
        };
        MessageResult(())
    }
}

impl Handler<GetPeer> for Network {
    type Result = MessageResult<GetPeer>;

    fn handle(&mut self, msg: GetPeer, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(index) = self.get_peer_index(&msg.0) {
            if let Some(val) = self.peer.get(index) {
                return MessageResult(Some(val.to_owned()));
            }
        }
        MessageResult(None)
    }
}

impl Handler<DumpPeer> for Network {
    type Result = MessageResult<DumpPeer>;

    fn handle(&mut self, _msg: DumpPeer, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.peer.clone())
    }
}

impl Handler<GetCurrentSize> for Network {
    type Result = MessageResult<GetCurrentSize>;

    fn handle(&mut self, _msg: GetCurrentSize, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.peer.len())
    }
}

impl Handler<ReplacePeerLedger> for Network {
    type Result = ();

    fn handle(&mut self, msg: ReplacePeerLedger, _ctx: &mut Self::Context) -> Self::Result {
        self.peer = msg.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[actix_rt::test]
    async fn network_actor_works() {
        let network_addr = Network::default().start();

        let peer = Peer {
            id: "one.example.com".into(),
            ip: "1.1.1.1:8080".into(),
        };

        // checks if genesis block can be appended to a blockchian
        network_addr.send(AddPeer(peer.clone())).await.unwrap();

        // checks if valid blocks can be added to blockchian
        assert_eq!(
            network_addr
                .send(GetPeer("one.example.com".into()))
                .await
                .unwrap()
                .unwrap()
                .ip,
            peer.ip,
            "add peer and get peer works"
        );

        // checks if invalid block, where block.get_prev() != network.get_last_block().get_hash()
        // can be added to network
        assert_eq!(
            network_addr.send(DumpPeer).await.unwrap().pop().unwrap().ip,
            peer.ip,
            "dump works"
        );

        // checking if GetCurrentSize works. At this point in the test, we already have
        // a peer enrolled, so we should see size == 1

        assert_eq!(
            network_addr.send(GetCurrentSize).await.unwrap(),
            1,
            "GetCurrentSize works"
        );
    }
}
