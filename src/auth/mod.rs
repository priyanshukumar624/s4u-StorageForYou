use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use dotenvy::var;

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let api_key = var("API_KEY").expect("API_KEY must be set");
    
    if credentials.token() == api_key {
        Ok(req)
    } else {
        Err((actix_web::error::ErrorUnauthorized("Invalid API key"), req))
    }
}