fn gen_hash(data: &str) -> String {
    use hex_literal::hex;
    use sha2::{Sha256, Digest};
    let result = Sha256::digest(data.as_bytes());
    hex::encode(result)
}

struct MerkleTree {
    pub leaves: Vec<Vec<u8>>,
    pub levels: Vec<Vec<Vec<u8>>>,
}

impl MerkleTree {
    fn new(data: Vec<&str>) -> Self {
        let mut leaves = Vec::new();
        let mut levels = Vec::new();
        for item in data {
            let leaves: Vec<Vec<u8>> = item.as_bytes().to_vec();
            let hash = gen_hash(item);
            leaves.push(hash.into_bytes());
        }

        levels.push(leaves.clone());
        Self { leaves, levels }
    }
}
