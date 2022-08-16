use jsonwebtoken::{decode, DecodingKey, Validation};
use rocket::{request::{FromRequest, self, Outcome}, Request, http::Status};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Claims {
    pub exp: i64,
    pub email: String,
    pub role: String,
    pub authorized: bool
}

pub struct JwtToken {
    pub email: String,
    pub token: String
}

#[derive(Debug)]
pub enum JwtTokenError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtToken {
    type Error = JwtTokenError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let auth_header: String = req.headers().get("Authorization").collect();

        if auth_header == "" {
            return Outcome::Failure((Status::Unauthorized, JwtTokenError::Missing));
        } else if auth_header.contains("Bearer") {
            let parts: Vec<&str> = auth_header.split(" ").collect();
            let token = parts.get(1).unwrap().to_string();
            let token_data = decode::<Claims>(
                &token,
                &DecodingKey::from_secret("ovojetajna".as_ref()),
                &Validation::default());
            let res = match token_data {
                Ok(res) => {
                    if res.claims.role != "ROLE_USER" {
                        return rocket::outcome::Outcome::Failure((Status::Unauthorized, JwtTokenError::Invalid));
                    }
                    
                    rocket::outcome::Outcome::Success(JwtToken{email: res.claims.email, token: token})
                },
                Err(_) => rocket::outcome::Outcome::Failure((Status::Unauthorized, JwtTokenError::Invalid)),
            };

            return res;
        }
        
        rocket::outcome::Outcome::Failure((Status::Unauthorized, JwtTokenError::Invalid))
    }
}

