use uuid::Uuid;
use rand::Rng;

pub fn generate_token() -> String {
    let random: u64 = rand::thread_rng().r#gen();
    format!("{}-{}", Uuid::new_v4(), random)
}
