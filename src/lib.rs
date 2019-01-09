pub mod hashing;

#[derive(Default)]
pub struct MerkleTree {
    pub root_hash: [u8; 32],
    pub n_nodes: u32,
    pub edge_nodes: Vec<[u8; 32]>
}

impl MerkleTree {
    pub fn insert(&mut self, value: &str) {
        let value_hash = hashing::blake2b_256(value.as_bytes());

        let mut pair_hash = value_hash;
        let mut new_edge = value_hash;
        let next_edge_addition_level = self.count_bit_set_from_right(self.n_nodes);

        for i in 0..self.edge_nodes.len() {
            let edge_node = self.edge_nodes[i];
            if edge_node == [0; 32] {
                pair_hash = hashing::blake2b_256(&pair_hash);
            } else {
                pair_hash = hashing::blake2b_256_pair(&edge_node, &pair_hash);
            }
            if (i + 1) as u8 == next_edge_addition_level {
                new_edge = pair_hash;
            }
        }
        self.root_hash = pair_hash;
        self.n_nodes += 1;
        self.update_edges(new_edge, next_edge_addition_level as usize);
    }

    fn update_edges(&mut self, new_edge_value: [u8; 32], addition_at_level: usize) {
        if addition_at_level >= self.edge_nodes.len() {
            self.edge_nodes.push(new_edge_value);
        } else {
            self.edge_nodes[addition_at_level] = new_edge_value;
        }

        for i in 0..addition_at_level {
            self.edge_nodes[i] = [0; 32];
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
