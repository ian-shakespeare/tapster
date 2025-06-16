use axum::http::{HeaderMap, StatusCode};
use chrono::{Duration, NaiveDateTime, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{model::UserModel, Error};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: i64,
    sub: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    #[serde(rename = "accessKey")]
    access_key: String,
    expiration: NaiveDateTime,
}

impl Auth {
    pub fn create(signing_key: &str, user: UserModel) -> crate::Result<Self> {
        let expiration = Utc::now() + Duration::weeks(2);
        let claims = Claims {
            exp: expiration.timestamp(),
            sub: user.id,
        };

        let access_key = jsonwebtoken::encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(signing_key.as_ref()),
        )?;

        Ok(Self {
            expiration: expiration.naive_utc(),
            access_key,
        })
    }

    pub fn decode(signing_key: &str, token: impl Into<String>) -> crate::Result<Uuid> {
        let decoded = jsonwebtoken::decode::<Claims>(
            &token.into(),
            &DecodingKey::from_secret(signing_key.as_ref()),
            &Validation::default(),
        )?;

        Ok(decoded.claims.sub)
    }

    pub fn decode_header(signing_key: &str, header: HeaderMap) -> crate::Result<Uuid> {
        let auth_header = header
            .get("authorization")
            .ok_or(Error::new(StatusCode::UNAUTHORIZED, "missing auth token"))?
            .to_str()
            .or(Err(Error::new(
                StatusCode::UNAUTHORIZED,
                "invalid auth token",
            )))?
            .to_string();

        let token = auth_header
            .split(' ')
            .skip(1)
            .next()
            .ok_or(Error::new(StatusCode::UNAUTHORIZED, "missing auth token"))?;

        Self::decode(signing_key, token)
    }
}
