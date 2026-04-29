use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn jwt_encode(user_id: String, secret: &str) -> String {
    let claims = Claims {
        sub: user_id,
        exp: 2000000000,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}

pub fn jwt_decode(token: String, secret: &str) -> String {
    let data = decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()).unwrap();

    data.claims.sub
}
