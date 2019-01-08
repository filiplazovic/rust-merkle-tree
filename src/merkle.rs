mod hashing;

pub struct MerkleTree {
    root_hash: [u8; 32],
    n_nodes: u32,
    edge_nodes: Vec<[u8; 32]>
}

impl MerkleTree {
    fn insert(&mut self, value: &str) {
        self.n_nodes += 1;
        let value_hash = hashing::blake2_256(value.as_bytes());

        let mut pair_hash = value_hash;
        let next_edge_level = self.get_next_edge_level(self.n_nodes);
        let mut new_edge = value_hash;
        for i in 0..self.edge_nodes.len() {
            let edge_node = self.edge_nodes[i];
            if edge_node == [0; 32] {
                pair_hash = hashing::blake2_256(&edge_node);
            } else {
                pair_hash = hashing::blake2b_256_pair(&edge_node, &pair_hash);
            }

            if (i + 1) as u8 == next_edge_level {
                new_edge = pair_hash;
            }
        }

        self.root_hash = pair_hash;
        self.update_edges(new_edge);
    }

    fn update_edges(&mut self, new_edge_value: [u8; 32]) {
        let previous_n_nodes = self.n_nodes - 1;

        let mut num_levels = self.edge_nodes.len();

        if (self.n_nodes & (self.n_nodes - 1)) == 0 {
            num_levels += 1;
        }


        for i in 0..num_levels {
            // Is casting `usize` to `u32` safe?
            let num_nodes_on_level = 2u32.pow(num_levels as u32);

            let current_contains = (num_nodes_on_level & self.n_nodes) != 0;
            let previous_contains = (num_nodes_on_level & previous_n_nodes) != 0;

            if i >= self.edge_nodes.len() {
                self.edge_nodes.push(new_edge_value);
            } else if current_contains && !previous_contains {
                self.edge_nodes[i] = new_edge_value;
            } else if !current_contains {
                self.edge_nodes[i] = [0; 32];
            }
        }
    }

    fn get_next_edge_level(&self, mut n_nodes: u32) -> u8 {
        let mut len: u8 = 0;
        while (n_nodes & 1) > 0 {
            n_nodes >>= 1;
            len += 1;
        }
        return len;
    }
}
