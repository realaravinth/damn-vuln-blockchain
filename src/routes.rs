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

use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};

use damn_vuln_blockchain::config::{Config, GetMode, Mode, SetMode};
use damn_vuln_blockchain::payload::{GetStake as PayloadGetStake, Peer, SellAsset};
use damn_vuln_blockchain::Client;

//#[post("/assets/buy")]
//async fn asset_buy(data: web::Data<Config>) -> impl Responder {
//}

// peer enrollment
#[post("/peer/enroll")]
async fn peer_enroll(peer: web::Json<Peer>, data: web::Data<Config>) -> impl Responder {
    use damn_vuln_blockchain::asset::InitNetworkBuilder;
    use damn_vuln_blockchain::discovery::{AddPeer, GetCurrentSize};

    // peer enrollment should only happen when current_network_size < data.init_network_size
    let current_network_size = data.network_addr.send(GetCurrentSize).await.unwrap();
    if current_network_size < data.init_network_size {
        let asset_message = InitNetworkBuilder::default()
            .network_size(data.init_network_size)
            .peer_id(peer.id.clone())
            .build()
            .unwrap();

        data.network_addr
            .send(AddPeer(peer.into_inner()))
            .await
            .unwrap();

        data.asset_addr.send(asset_message).await.unwrap();
    }

    // TODO, must return error when current_network_size == data.init_network_size
    HttpResponse::Ok()
}

// peer enrollment
#[get("/peer/discover/all")]
async fn peer_dump(data: web::Data<Config>) -> impl Responder {
    use damn_vuln_blockchain::discovery::DumpPeer;
    let peer_data = data.network_addr.send(DumpPeer).await.unwrap();
    HttpResponse::Ok().json(peer_data)
}

// asset dump
#[get("/assets/all")]
async fn assets_dump(data: web::Data<Config>) -> impl Responder {
    use damn_vuln_blockchain::asset::DumpLedger;
    let assets = data.asset_addr.send(DumpLedger).await.unwrap();
    HttpResponse::Ok().json(assets)
}

// attack
#[post("/attack")]
async fn set_attack(data: web::Data<Config>) -> impl Responder {
    if data.mode_addr.send(GetMode).await.unwrap() == Mode::Attacker(false) {
        data.mode_addr
            .send(SetMode(Mode::Attacker(true)))
            .await
            .unwrap();
    }
    HttpResponse::Ok()
}

// get stake for a particular block ID
#[post("/stake")]
async fn get_stake(payload: web::Json<PayloadGetStake>, data: web::Data<Config>) -> impl Responder {
    use damn_vuln_blockchain::asset::{GetStake as ActorGetStake, SetStakeBuilder};
    let msg: ActorGetStake = payload.into_inner().into();
    // attacking peer should always return stake = 0
    if data.mode_addr.send(GetMode).await.unwrap() == Mode::Attacker(false) {
        let set_stake_msg = SetStakeBuilder::default()
            .block_id(msg.0)
            .peer_id(data.peer_id.clone())
            .stake(Vec::default())
            .build()
            .unwrap();
        data.asset_addr.send(set_stake_msg).await.unwrap();
    };

    let stake = data.asset_addr.send(msg).await.unwrap();

    HttpResponse::Ok().json(stake)
}

// buy asset
#[post("/assets/sell")]
async fn sell(
    client: web::Data<Client>,
    payload: web::Json<SellAsset>,
    data: web::Data<Config>,
) -> impl Responder {
    use damn_vuln_blockchain::asset::{ChooseValidator, GetAssetInfo, Stake};
    use damn_vuln_blockchain::chain::GetLastBlock;
    use damn_vuln_blockchain::client::GetStake as ClientGetStake;
    use damn_vuln_blockchain::discovery::{DumpPeer, GetPeer};

    if let Some(asset_info) = data
        .asset_addr
        .send(GetAssetInfo(payload.asset_id.clone()))
        .await
        .unwrap()
    {
        if let Some(owner) = asset_info.get_owner() {
            if owner != &data.peer_id {
                // stake must be custom, the below
                // valudation selection doesn't work
                // I must get stake from all peers and the
                // choose validator
                // Steps:
                // 1. Get stake from all(peer: if unspecified, return full stake)
                // 2. Choose validator
                // 3. Send transaction request
                //
                // maybe AssetLedger can have a second structure with stake
                // for every block ID?
                //            let validator = data
                //                .asset_addr
                //                .send(ChooseValidator)
                //                .await
                //                .unwrap()
                //                // unwrap below should be taken care of
                //                // None occurs when there are no peers in
                //                // the network
                //                .unwrap();
                //            let validator_peer = data
                //                .network_addr
                //                .send(GetPeer(validator))
                //                .await
                //                .unwrap()
                //                .unwrap();
                //            //TODO:
                //            // 1. send peer the transaction request
                let mut stake: Vec<Stake> = Vec::new();
                let peers = data.network_addr.send(DumpPeer).await.unwrap();
                let current_block = data.chain_addr.send(GetLastBlock).await.unwrap();
                let next_block_id = current_block.get_serial_no().unwrap() + 1;
                peers.iter().for_each(|peer| {
                    let a = async {
                        let client_payload = ClientGetStake {
                            block_id: next_block_id,
                            peer_id: peer.id.clone(),
                        };
                        //client.get_stake(client_payload, &data).await;
                    };
                });
            }
        }
    };

    HttpResponse::Ok()
}

pub fn services(cfg: &mut ServiceConfig) {
    cfg.service(peer_enroll);
    cfg.service(peer_dump);
    cfg.service(assets_dump);
    cfg.service(get_stake);
    cfg.service(set_attack);
}
