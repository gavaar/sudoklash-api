// todo: refactor
use std::future::{ready, Ready};
use actix_web::{
    dev::Payload,
    error::{Error as ActixWebError, ErrorUnauthorized},
    http, web, FromRequest, HttpRequest,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde_json::json;

use crate::models::{AppState, TokenClaims};

pub struct AuthenticationGuard {
    pub user_id: String,
}
impl FromRequest for AuthenticationGuard {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token = req
            .cookie("token")
            .map(|c| c.value().to_owned())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_owned())
            });

        if token.is_none() {
            return ready(Err(ErrorUnauthorized(
                json!({"status": "fail", "message": "You are not logged in, please provide token"}),
            )));
        }

        let data: &web::Data<AppState> = req.app_data::<web::Data<AppState>>().unwrap();
        let jwt_secret = data.env.jwt_secret.to_owned();
        println!("{}", token.to_owned().unwrap().as_str());
        let decode = decode::<TokenClaims>(
            token.unwrap().as_str(),
            &DecodingKey::from_secret(jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        );

        match decode {
            Ok(token) => {
                let vec = data.db.lock().unwrap();
                let user = vec
                    .iter()
                    .find(|user| user.id == Some(token.claims.sub.to_owned()));

                if user.is_none() {
                    return ready(Err(ErrorUnauthorized(
                        json!({"status": "fail", "message": "User belonging to this token no logger exists"}),
                    )));
                }

                ready(Ok(AuthenticationGuard {
                    user_id: token.claims.sub,
                }))
            }
            Err(_) => ready(Err(ErrorUnauthorized(
                json!({"status": "fail", "message": "Invalid token or user doesn't exists"}),
            ))),
        }
    }
}
