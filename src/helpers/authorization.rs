use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest, Error, FromRequest};
use actix_web::error::ErrorUnauthorized;
use std::future::{Ready, ready};

use crate::helpers::config::Config;

pub struct Authorized;
impl FromRequest for Authorized {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        // Access the application state (Config)
        if let Some(config) = req.app_data::<web::Data<Config>>() {
            let auth_header = req.headers().get("Authorization").cloned();
            
            return ready(match auth_header {
                Some(header_value) => {
                    if header_value == format!("Bearer {}", config.auth_token) {
                        Ok(Authorized)
                    } else {
                        Err(ErrorUnauthorized("Invalid token"))
                    }
                },
                None => Err(ErrorUnauthorized("No token found")),
            });
        } else {
            // Config not found in app data
            return ready(Err(ErrorUnauthorized("Configuration not found")));
        }
    }
}