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
    get,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};

use crate::Config;
use damn_vuln_blockchain::logs::Peer;

//#[get("/")]
//async fn greet(req: HttpRequest) -> impl Responder {
//    let name = req.match_info().get("name").unwrap_or("World");
//    format!("Hello {}!", &name)
//}

// peer enrollment
#[get("/peer/enroll/")]
async fn peer_enroll(creds: web::Json<Peer>, data: web::Data<Config>) -> impl Responder {
    //    let ip = creds.into_inner().ip;
    //    let peer_id = creds.into_inner().id;
    HttpResponse::Ok()
}

pub fn services(cfg: &mut ServiceConfig) {
    //    cfg.service(greet);
}
