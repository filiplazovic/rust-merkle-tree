use hashing::{blake2b_256, blake2b_256_pair, Hash};

pub struct MerkleTree {
    pub root_hash: Option<Hash>,
    pub n_nodes: u32,
    pub edge_nodes: Vec<Option<Hash>>
}

impl MerkleTree {
    pub fn new() -> MerkleTree {
        MerkleTree {
            root_hash: None,
            n_nodes: 0,
            edge_nodes: vec![]
        }
    }

    pub fn insert(&mut self, value: &str) {
        let value_hash = blake2b_256(value.as_bytes());

        let mut pair_hash = value_hash;
        let mut new_edge = value_hash;
        let next_edge_addition_level = self.count_bit_set_from_right(self.n_nodes);

        for i in 0..self.edge_nodes.len() {
            let edge_node = self.edge_nodes[i];
            pair_hash = match edge_node {
                Some(hash) => blake2b_256_pair(&hash, &pair_hash),
                None => blake2b_256(&pair_hash)
            };
            if (i + 1) as u8 == next_edge_addition_level {
                new_edge = pair_hash;
            }
        }
        self.root_hash = Some(pair_hash);
        self.n_nodes += 1;
        self.update_edges(new_edge, next_edge_addition_level as usize);
    }

    fn update_edges(&mut self, new_edge_value: Hash, addition_at_level: usize) {
        if addition_at_level >= self.edge_nodes.len() {
            self.edge_nodes.push(Some(new_edge_value));
        } else {
            self.edge_nodes[addition_at_level] = Some(new_edge_value);
        }

        for i in 0..addition_at_level {
            self.edge_nodes[i] = None;
        }
    }

    fn count_bit_set_from_right(&self, mut num: u32) -> u8 {
        let mut len: u8 = 0;
        while (num & 1) > 0 {
            num >>= 1;
            len += 1;
        }
        return len;
    }
}
