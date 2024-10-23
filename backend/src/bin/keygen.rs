use base64::{Engine, engine::general_purpose};
use jwt_simple::prelude::*;

fn main() {
    let key_pair_bytes = ES256KeyPair::generate().to_bytes();
    println!("Base64 ES256KeyPair: {}", general_purpose::STANDARD.encode(key_pair_bytes));
}