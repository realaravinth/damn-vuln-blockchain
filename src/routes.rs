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
use log::debug;

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
    let current_mode = data.mode_addr.send(GetMode).await.unwrap();
    let new_mode = match current_mode {
        Mode::Attacker(val) => Some(Mode::Attacker(!val)),
        Mode::Victim(val) => Some(Mode::Victim(!val)),
        _ => None,
    };

    if let Some(mode) = new_mode {
        data.mode_addr.send(SetMode(mode)).await.unwrap();
    }
    HttpResponse::Ok()
}

// get stake for a particular block ID
#[post("/stake")]
async fn get_stake(payload: web::Json<PayloadGetStake>, data: web::Data<Config>) -> impl Responder {
    use damn_vuln_blockchain::asset::{GetStake as ActorGetStake, SetStakeBuilder};
    let msg: ActorGetStake = payload.into_inner().into();
    // attacking peer should always return stake = 0
    let current_mode = data.mode_addr.send(GetMode).await.unwrap();

    if current_mode == Mode::Attacker(false) || current_mode == Mode::Attacker(true) {
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
    use damn_vuln_blockchain::asset::GetAssetInfo;
    use damn_vuln_blockchain::chain::GetLastBlock;
    use damn_vuln_blockchain::utils::consensus;

    if let Some(asset_info) = data
        .asset_addr
        .send(GetAssetInfo(payload.asset_id.clone()))
        .await
        .unwrap()
    {
        debug!("Owner: {:#?}", asset_info.get_owner());
        if let Some(owner) = asset_info.get_owner() {
            if owner == &data.peer_id {
                debug!("Ownership verified");

                let current_block = data.chain_addr.send(GetLastBlock).await.unwrap();
                let next_block_id = current_block.get_serial_no().unwrap() + 1;

                let validator = consensus(&data, next_block_id, &client).await;
            }
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
