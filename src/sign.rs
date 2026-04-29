use ed25519_dalek::{SigningKey, VerifyingKey, Signer, Signature};
use rand::rngs::OsRng;
use ed25519_dalek::Verifier;
use rand::RngCore;

pub fn sign_message(msg: String) -> String {
    let mut csprng = OsRng;

    let mut key_bytes = [0u8; 32];
    csprng.fill_bytes(&mut key_bytes);

    let signing_key = SigningKey::from_bytes(&key_bytes);
    let verify_key = signing_key.verifying_key();

    let signature = signing_key.sign(msg.as_bytes());

    format!(
        "{}::{}",
        hex::encode(signature.to_bytes()),
        hex::encode(verify_key.to_bytes())
    )
}

pub fn verify_message(msg: String, data: String) -> bool {
    let parts: Vec<&str> = data.split("::").collect();

    if parts.len() != 2 {
        return false;
    }

    let sig_bytes = match hex::decode(parts[0]) {
        Ok(v) => v,
        Err(_) => return false,
    };

    let pub_bytes = match hex::decode(parts[1]) {
        Ok(v) => v,
        Err(_) => return false,
    };

    let signature = match sig_bytes.as_slice().try_into() {
        Ok(arr) => Signature::from_bytes(&arr),
        Err(_) => return false,
    };

    let verify_key = match pub_bytes.as_slice().try_into() {
        Ok(arr) => match VerifyingKey::from_bytes(&arr) {
            Ok(k) => k,
            Err(_) => return false,
        },
        Err(_) => return false,
    };

    verify_key.verify(msg.as_bytes(), &signature).is_ok()
}