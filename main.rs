use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
struct TransformRequest {
    xml_data: String,
    mapping: Option<HashMap<String, String>>,
}

async fn transform(req: web::Json<TransformRequest>, _req_head: HttpRequest) -> impl Responder {
    // Parse XML using serde_xml_rs
    let adf: serde_xml_rs::Value = match from_str(&req.xml_data) {
        Ok(val) => val,
        Err(e) => return HttpResponse::BadRequest().body(format!("Invalid XML: {}", e)),
    };

    // Default mapping if none is provided
    let default_mapping = HashMap::from([
        ("requestDate".to_string(), "prospect.requestdate".to_string()),
        ("customerName".to_string(), "prospect.customer.contact.name".to_string()),
        ("vendorName".to_string(), "prospect.vendor.vendorname".to_string()),
        ("vehicleMake".to_string(), "prospect.vehicle.make".to_string()),
        ("vehicleModel".to_string(), "prospect.vehicle.model".to_string()),
    ]);

    // Use provided mapping or the default
    let mapping = req.mapping.clone().unwrap_or(default_mapping);

    // Apply mapping to extract data
    let mut result = serde_json::Map::new();
    for (json_key, xml_path) in mapping.iter() {
        if let Some(val) = adf.pointer(xml_path) {
            result.insert(json_key.to_string(), val.clone());
        } 
    }

    HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/transform")
                    .route(web::post().to(transform)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
