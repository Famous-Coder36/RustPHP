use sha2::{Sha256, Digest};

pub fn sha256(text: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text);
    format!("{:x}", hasher.finalize())
}

pub fn blake3_hash(text: String) -> String {
    blake3::hash(text.as_bytes()).to_hex().to_string()
}	
