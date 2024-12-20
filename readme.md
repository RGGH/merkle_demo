```markdown
# Merkle Tree in Rust

This project demonstrates how to build a Merkle tree in Rust. Merkle trees are widely used in blockchain and cryptographic systems to enable efficient and secure verification of large datasets.

## How It Works

1. **Hash Leaves**: Each piece of data (leaf node) is hashed.
2. **Handle Odd Nodes**: If the number of nodes in a layer is odd, duplicate the last node.
3. **Combine and Hash Pairs**: Concatenate each pair of nodes, hash them, and create a new layer.
4. **Repeat**: Continue combining nodes layer by layer until a single root node remains.
5. **Output the Root**: The final node is the Merkle tree root.

## Code Example

```rust
use sha2::{Sha256, Digest};

fn hash_data(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    format!("{:x}", hasher.finalize()) // Convert to hex string
}

fn merkle_tree(leaves: Vec<&str>) -> String {
    let mut layer: Vec<String> = leaves.iter().map(|&leaf| hash_data(leaf)).collect();

    while layer.len() > 1 {
        if layer.len() % 2 != 0 {
            layer.push(layer.last().unwrap().clone());
        }

        let mut new_layer: Vec<String> = Vec::new();
        for i in (0..layer.len()).step_by(2) {
            new_layer.push(hash_data(&format!("{}{}", layer[i], layer[i + 1])));
        }

        layer = new_layer;
    }

    layer[0].clone()
}

fn main() {
    let leaves = vec!["data1", "data2", "data3", "data4", "data5", "data6", "data7", "data8"];
    let root_hash = merkle_tree(leaves);
    println!("Merkle tree root hash: {}", root_hash);
}
```

## How the Code Works

- **Hashing**: The `hash_data` function uses SHA-256 to hash the data.
- **Tree Construction**: The `merkle_tree` function builds the tree by hashing pairs of nodes until only one root remains.
- **Main Function**: Provides a sample dataset, constructs the Merkle tree, and prints the root hash.


