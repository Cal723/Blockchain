fn gen_hash(data: &[u8]) -> Vec<u8> {
    let result = Sha256::digest(data);
    result.to_vec() 
}

//Merkle Struct
struct MerkleTree {
    pub leaves: Vec<Vec<u8>>,
    pub levels: Vec<Vec<Vec<u8>>>,
    pub root: Vec<u8>,
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

        let mut cur_level = leaves.clone();

        //assuming data.len() is power of 2
        while cur_level.len() > 1 {
            let mut new_level = Vec::new();
            for i in (0..cur_level.len()).step_by(2) {
                let first = &cur_level[i];
                let second = &cur_level[i + 1];
                let both = [first.as_slice(), second.as_slice()].concat();
                let new_hash = gen_hash(&both);
                new_level.push(new_hash);
            }
            cur_level = new_level;
            levels.push(new_level.clone());
        }
        root = cur_level[0].clone();
        
        Self { leaves, levels, root }
    }

}