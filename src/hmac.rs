use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn hmac_sign(data: String, key: String) -> String {
    let mut mac = HmacSha256::new_from_slice(key.as_bytes()).unwrap();
    mac.update(data.as_bytes());

    hex::encode(mac.finalize().into_bytes())
}

pub fn hmac_verify(data: String, key: String, hash: String) -> bool {
    let new_hash = hmac_sign(data, key);
    new_hash == hash
}
