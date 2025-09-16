fn gen_hash(data: &str) -> String {
    use hex_literal::hex;
    use sha2::{Sha256, Digest};
    let result = Sha256::digest(data.as_bytes());
    hex::encode(result)
}

struct MerkleTree {
    root: String,
    leaves: Vec<string>,
    
}
