extern crate merkle_tree;

use merkle_tree::{MerkleTree};
use merkle_tree::hashing::{blake2b_256, blake2b_256_pair};

#[test]
fn should_initialize_merkle_tree() {
    let merkle_tree: MerkleTree = Default::default();
    assert_eq!(merkle_tree.root_hash, [0; 32]);
    assert_eq!(merkle_tree.n_nodes, 0);
    assert_eq!(merkle_tree.edge_nodes.len(), 0);
}

#[test]
fn should_correctly_calculate_root_hash_and_edge_nodes() {
    let mut merkle_tree: MerkleTree = Default::default();

    // a
    let a = "a".to_string();
    let a_hash = blake2b_256(a.as_bytes());

    merkle_tree.insert(&a);

    let mut root_hash = &merkle_tree.root_hash;
    let mut edge_nodes = &merkle_tree.edge_nodes;

    assert_eq!(root_hash, &a_hash);
    assert_eq!(edge_nodes, &vec![a_hash]);

    // b
    let b = "b".to_string();
    let b_hash = blake2b_256(b.as_bytes());

    let ab_hash = blake2b_256_pair(&a_hash, &b_hash);

    merkle_tree.insert(&b);

    root_hash = &merkle_tree.root_hash;
    edge_nodes = &merkle_tree.edge_nodes;

    assert_eq!(root_hash, &ab_hash);
    assert_eq!(edge_nodes, &vec![[0; 32], ab_hash]);

    // c
    let c = "c".to_string();
    let c_hash = blake2b_256(c.as_bytes());

    let c1_hash = blake2b_256(&c_hash);

    let abc1_hash = blake2b_256_pair(&ab_hash, &c1_hash);

    merkle_tree.insert(&c);

    root_hash = &merkle_tree.root_hash;
    edge_nodes = &merkle_tree.edge_nodes;

    assert_eq!(root_hash, &abc1_hash);
    assert_eq!(edge_nodes, &vec![c_hash, ab_hash]);

    // d
    let d = "d".to_string();
    let d_hash = blake2b_256(d.as_bytes());

    let cd_hash = blake2b_256_pair(&c_hash, &d_hash);

    let abcd_hash = blake2b_256_pair(&ab_hash, &cd_hash);

    merkle_tree.insert(&d);

    root_hash = &merkle_tree.root_hash;
    edge_nodes = &merkle_tree.edge_nodes;

    assert_eq!(root_hash, &abcd_hash);
    assert_eq!(edge_nodes, &vec![[0; 32], [0; 32], abcd_hash]);

    // e
    let e = "e".to_string();
    let e_hash = blake2b_256(e.as_bytes());

    let e1_hash = blake2b_256(&e_hash);

    let e2_hash = blake2b_256(&e1_hash);

    let abcde2_hash = blake2b_256_pair(&abcd_hash, &e2_hash);

    merkle_tree.insert(&e);

    root_hash = &merkle_tree.root_hash;
    edge_nodes = &merkle_tree.edge_nodes;

    assert_eq!(root_hash, &abcde2_hash);
    assert_eq!(edge_nodes, &vec![e_hash, [0; 32], abcd_hash]);

    // f
    let f = "f".to_string();
    let f_hash = blake2b_256(f.as_bytes());

    let ef_hash = blake2b_256_pair(&e_hash, &f_hash);

    let ef1_hash = blake2b_256(&ef_hash);

    let abcdef1_hash = blake2b_256_pair(&abcd_hash, &ef1_hash);

    merkle_tree.insert(&f);

    root_hash = &merkle_tree.root_hash;
    edge_nodes = &merkle_tree.edge_nodes;

    assert_eq!(root_hash, &abcdef1_hash);
    assert_eq!(edge_nodes, &vec![[0; 32], ef_hash, abcd_hash]);

    // g
    let g = "g".to_string();
    let g_hash = blake2b_256(g.as_bytes());

    let g1_hash = blake2b_256(&g_hash);

    let efg1_hash = blake2b_256_pair(&ef_hash, &g1_hash);

    let abcdefg1_hash = blake2b_256_pair(&abcd_hash, &efg1_hash);

    merkle_tree.insert(&g);

    root_hash = &merkle_tree.root_hash;
    edge_nodes = &merkle_tree.edge_nodes;

    assert_eq!(root_hash, &abcdefg1_hash);
    assert_eq!(edge_nodes, &vec![g_hash, ef_hash, abcd_hash]);

    // h
    let h = "h".to_string();
    let h_hash = blake2b_256(h.as_bytes());

    let gh_hash = blake2b_256_pair(&g_hash, &h_hash);
    let efgh_hash = blake2b_256_pair(&ef_hash, &gh_hash);

    let abcdefgh_hash = blake2b_256_pair(&abcd_hash, &efgh_hash);

    merkle_tree.insert(&h);

    root_hash = &merkle_tree.root_hash;
    edge_nodes = &merkle_tree.edge_nodes;

    assert_eq!(root_hash, &abcdefgh_hash);
    assert_eq!(edge_nodes, &vec![[0; 32], [0; 32], [0; 32], abcdefgh_hash]);

    // i
    let i = "i".to_string();
    let i_hash = blake2b_256(i.as_bytes());

    let i1_hash = blake2b_256(&i_hash);

    let i2_hash = blake2b_256(&i1_hash);

    let i3_hash = blake2b_256(&i2_hash);

    let abcdefghi3_hash = blake2b_256_pair(&abcdefgh_hash, &i3_hash);

    merkle_tree.insert(&i);

    root_hash = &merkle_tree.root_hash;
    edge_nodes = &merkle_tree.edge_nodes;

    assert_eq!(root_hash, &abcdefghi3_hash);
    assert_eq!(edge_nodes, &vec![i_hash, [0; 32], [0; 32], abcdefgh_hash]);
}
