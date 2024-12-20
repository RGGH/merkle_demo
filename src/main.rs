use sha2::{Sha256,Digest};

fn hash_data(data:&str)->String{
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    format!("{:x}",result)

}

fn merkle_tree(leaves:Vec<&str>)->String{
    // step 1 - Hash all of the leaves, to create the 1st layer of the tree
    let mut layer:Vec<String> = leaves.iter().map(|leaf|hash_data(leaf)).collect();
    
    // Step 2-4: Build the tree upwards until a single node remains
    while layer.len() > 1 {
        if layer.len() %2==1 {
            // Step 2: If there's an odd number of nodes, duplicate the last one
            layer.push(layer.last().unwrap().clone());
        }

    // Step 3: Combine and hash pairs of nodes
    let mut new_layer: Vec<String> = Vec::new();
    for i in (0..layer.len()).step_by(2){
        let combined = format!("{}{}",layer[i],layer[i+1]);
        new_layer.push(hash_data(&combined));
    }
    layer = new_layer;
    }
    // Step 5: The remaining hash is the root of the Merkle tree
    // the layer vector contains exactly one element
    // When you access elements like above..layer.last(), 
    // you get a reference (&String), so you need to clone it 
    // to take ownership of it when pushing it to the layer
    layer[0].clone()

}





fn main(){
    let leaves = vec!["data1","data2","data3","data4", 
                      "data5","data6","data7","data8"];

    let root_hash = merkle_tree(leaves);
    println!("{:?}", root_hash);

}
