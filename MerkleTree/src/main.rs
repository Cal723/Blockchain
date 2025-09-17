fn gen_hash(data: &[u8]) -> Vec<u8> {
    let result = Sha256::digest(data);
    result.to_vec() 
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
            let hash = gen_hash(item.as_bytes());
            leaves.push(hash); 
        }
        levels.push(leaves.clone());
        Self { leaves, levels }
    }

}
