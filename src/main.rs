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
mod routes;
#[cfg(test)]
mod tests;

#[actix_web::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> std::io::Result<()> {
    use actix_web::{
        middleware::{normalize, Compress, Logger, NormalizePath},
        App, HttpServer,
    };
    use damn_vuln_blockchain::api::Client;
    use damn_vuln_blockchain::config::Config;

    pretty_env_logger::init();

    let config = Config::new();

    config.bootstrap().await;
    let ip_addr = config.public_ip.clone();

    HttpServer::new(move || {
        App::new()
            .configure(routes::services)
            .data(config.clone())
            .data(Client::default())
            .app_data(get_json_err())
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(NormalizePath::new(normalize::TrailingSlash::Trim))
    })
    .bind(ip_addr)?
    .run()
    .await
}

use actix_web::{error::InternalError, http::StatusCode, web::JsonConfig};
#[cfg(not(tarpaulin_include))]
fn get_json_err() -> JsonConfig {
    JsonConfig::default().error_handler(|err, _| {
        //debug!("JSON deserialization error: {:?}", &err);
        InternalError::new(err, StatusCode::BAD_REQUEST).into()
    })
}
