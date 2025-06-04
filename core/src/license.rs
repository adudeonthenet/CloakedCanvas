/*
 * CloakedCanvas - MIT License
 */

use serde::Serialize;
use jsonwebtoken::{EncodingKey, Header};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::error::Error;
use reqwest::blocking::Client;

#[derive(Serialize)]
struct Claims {
    exp: usize,
}

/// Perform a license heartbeat using mTLS and return the JWT.
pub fn license_heartbeat(server: &str, identity_pem: &[u8], secret: &[u8]) -> Result<String, Box<dyn Error>> {
    let client = Client::builder()
        .identity(reqwest::Identity::from_pem(identity_pem)?)
        .build()?;
    let exp = (SystemTime::now() + Duration::from_secs(7 * 24 * 3600))
        .duration_since(UNIX_EPOCH)?
        .as_secs() as usize;
    let token = jsonwebtoken::encode(&Header::default(), &Claims { exp }, &EncodingKey::from_secret(secret))?;
    let resp = client.post(server).body(token.clone()).send()?;
    if resp.status().is_success() {
        Ok(token)
    } else {
        Err("heartbeat failed".into())
    }
}
