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

use damn_vuln_blockchain::block::Block;
use damn_vuln_blockchain::config::{Config, GetMode, Mode, SetMode};
use damn_vuln_blockchain::payload::{
    GetStake as PayloadGetStake, Peer, Tx, ValidateTx, ValidateTxBuilder,
};
use damn_vuln_blockchain::Client;
use log::debug;

//#[post("/assets/buy")]
//async fn asset_buy(data: web::Data<Config>) -> impl Responder {
//}

// peer enrollment
#[post("/peers/enroll")]
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
#[get("/peers/all")]
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

// chain dump
#[get("/chain/all")]
async fn chain_dump(data: web::Data<Config>) -> impl Responder {
    use damn_vuln_blockchain::chain::DumpLedger;
    let chain = data.chain_addr.send(DumpLedger).await.unwrap();
    HttpResponse::Ok().json(chain)
}

// attack
#[post("/attack")]
async fn set_attack(data: web::Data<Config>) -> impl Responder {
    let current_mode = data.mode_addr.send(GetMode).await.unwrap();
    debug!("{}", &format!("current mode: {:?}", &current_mode));
    let new_mode = match current_mode {
        Mode::Attacker(val) => Some(Mode::Attacker(!val)),
        Mode::Victim(val) => Some(Mode::Victim(!val)),
        _ => None,
    };

    if let Some(mode) = new_mode {
        debug!("{}", &format!("changing mode to: {:?}", &mode));
        data.mode_addr.send(SetMode(mode)).await.unwrap();
    }
    HttpResponse::Ok()
}

// get stake for a particular block ID
#[post("/stake")]
async fn get_stake(payload: web::Json<PayloadGetStake>, data: web::Data<Config>) -> impl Responder {
    use damn_vuln_blockchain::asset::{GetStake as ActorGetStake, StakeBuilder};
    let msg: ActorGetStake = payload.into_inner().into();
    // attacking peer should always return stake = 0
    let current_mode = data.mode_addr.send(GetMode).await.unwrap();

    if current_mode == Mode::Attacker(false) || current_mode == Mode::Victim(true) {
        //        let set_stake_msg = SetStakeBuilder::default()
        //            .block_id(msg.0)
        //            .peer_id(data.peer_id.clone())
        //            .stake(Vec::default())
        //            .build()
        //            .unwrap();
        //        data.asset_addr.send(set_stake_msg).await.unwrap();
        let stake = StakeBuilder::default()
            .stake(Vec::default())
            .block_id(msg.0)
            .build()
            .unwrap();

        HttpResponse::Ok().json(stake)
    } else {
        let stake = data.asset_addr.send(msg).await.unwrap();

        HttpResponse::Ok().json(stake)
    }
}

// sell asset
#[post("/assets/sell")]
async fn sell(
    client: web::Data<Client>,
    payload: web::Json<Tx>,
    data: web::Data<Config>,
) -> impl Responder {
    use damn_vuln_blockchain::utils::{check_ownership, consensus, get_next_block_id};

    if check_ownership(&data, &data.peer_id, &payload.asset_id).await {
        let next_block_id = get_next_block_id(&data).await;
        let validator = consensus(&data, next_block_id, &client).await;
        let validator_payload = ValidateTxBuilder::default()
            .tx(payload.into_inner())
            .seller_peer_id(data.peer_id.clone())
            .build()
            .unwrap();
        client
            .send_tx_to_validator(&validator, &validator_payload)
            .await;
    } else {
        debug!("Ownership not verified");
    };

    HttpResponse::Ok()
}

// block add
#[post("/chain/add")]
async fn add_block(
    client: web::Data<Client>,
    payload: web::Json<Block>,
    data: web::Data<Config>,
) -> impl Responder {
    use damn_vuln_blockchain::utils::*;

    if check_ownership(
        &data,
        payload.get_tx().unwrap(),
        &payload.get_asset_id().unwrap(),
    )
    .await
    {
        add_block_runner(&data, &client, &payload).await;
    } else {
        debug!("Ownership not verified");
    };

    HttpResponse::Ok()
}

// validate and create block
#[post("/block/validate")]
async fn validate(
    client: web::Data<Client>,
    payload: web::Json<ValidateTx>,
    data: web::Data<Config>,
) -> impl Responder {
    use damn_vuln_blockchain::block::BlockBuilder;

    use damn_vuln_blockchain::chain::GetLastBlock;
    use damn_vuln_blockchain::utils::*;

    if check_ownership(&data, &payload.seller_peer_id, &payload.tx.asset_id).await {
        let next_block_id = get_next_block_id(&data).await;
        let validator = consensus(&data, next_block_id, &client).await;
        if data.peer_id == validator.id {
            debug!("Consensus verified, proceeding with block creation");
            // 1. Create block
            // 2. Change asset ownership
            // 3. mutate validation assets and sold last transaction
            // 4. add block to chain
            // 5. broadcast
            let last_block = data.chain_addr.send(GetLastBlock).await.unwrap();
            let new_block = BlockBuilder::default()
                .set_tx(&payload.seller_peer_id)
                .set_rx(&payload.tx.buyer_peer_id)
                .set_asset_id(&payload.tx.asset_id)
                .set_validator(&data.peer_id)
                .set_prev(&last_block)
                .build();
            add_block_runner(&data, &client, &new_block).await;
            broadcast_block(&data, &client, &new_block).await;
        } else {
            debug!("Consensus failure");
        }
    } else {
        debug!("Ownership not verified");
    };

    HttpResponse::Ok()
}

pub fn services(cfg: &mut ServiceConfig) {
    cfg.service(peer_enroll);
    cfg.service(peer_dump);
    cfg.service(assets_dump);
    cfg.service(get_stake);
    cfg.service(set_attack);
    cfg.service(sell);
    cfg.service(chain_dump);
    cfg.service(validate);
    cfg.service(add_block);
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use actix_web::{dev::ServiceResponse, http::header, test, App};

    #[cfg(test)]
    pub async fn make_post_request(
        config: &Config,
        payload: Option<String>,
        url: &str,
    ) -> ServiceResponse {
        let req;
        if payload.is_some() {
            req = test::TestRequest::post()
                .uri(url)
                .header(header::CONTENT_TYPE, "applicatin/json")
                .set_payload(payload.unwrap());
        } else {
            req = test::TestRequest::post()
                .uri(url)
                .header(header::CONTENT_TYPE, "applicatin/json")
        }
        let mut app = test::init_service(App::new().configure(services).data(config.clone())).await;
        test::call_service(&mut app, req.to_request()).await
    }

    #[cfg(test)]
    pub async fn make_get_request(config: &Config, url: &str) -> ServiceResponse {
        let req = test::TestRequest::get().uri(url).to_request();
        let mut app = test::init_service(App::new().configure(services).data(config.clone())).await;

        test::call_service(&mut app, req).await
    }
}
