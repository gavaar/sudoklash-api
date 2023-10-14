use std::future::{ready, Ready};
use actix_web::{
    dev::Payload,
    http, web, FromRequest, HttpRequest,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use crate::models::{AppState, TokenClaims, ErrorResponse};

pub struct AuthenticationGuard {
    pub user_id: String,
}

impl FromRequest for AuthenticationGuard {
    type Error = ErrorResponse;
    type Future = Ready<Result<Self, ErrorResponse>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token = req.headers()
            .get(http::header::AUTHORIZATION)
            .map(|h|
                h.to_str().unwrap().split_at(7).1.to_owned()
            );

        if let None = token {
            return ready(Err(ErrorResponse::Unauthorized("You are not logged in, please provide token".to_string())));
        }

        let db_data: &web::Data<AppState> = req.app_data::<web::Data<AppState>>().unwrap();
        let jwt_secret = db_data.env.jwt_secret.to_owned();
        let decode = decode::<TokenClaims>(
            token.unwrap().as_str(),
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        );

        match decode {
            Ok(token) => {
                let db_data = db_data.db.lock().unwrap();
                let user = db_data
                    .iter()
                    .find(|user| user.id == token.claims.sub.to_owned());

                if let None = user {
                    return ready (Err(ErrorResponse::Unauthorized(String::from("User was not found.. odd"))))
                }

                ready(Ok(AuthenticationGuard { user_id: token.claims.sub }))
            }
            Err(_) => ready(Err(ErrorResponse::Unauthorized(String::from("Invalid token or user doesn't exists")))),
        }
    }
}
