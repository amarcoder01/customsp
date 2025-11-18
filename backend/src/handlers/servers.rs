use actix_web::{web, HttpResponse, Result};
use log::info;

use crate::config::AppConfig;
use crate::models::{Location, ServerInfo};

pub async fn get_servers(config: web::Data<AppConfig>) -> Result<HttpResponse> {
    info!("Servers list requested");
    
    let servers = vec![ServerInfo {
        id: config.server_id.clone(),
        name: config.server_name.clone(),
        location: Location {
            lat: config.server_lat,
            lon: config.server_lon,
        },
        available: true,
        load: 0.3, // TODO: Calculate actual load
    }];
    
    Ok(HttpResponse::Ok().json(servers))
}
