use crate::index_structs::{index_tree::{add_to_tree, IndexTreeNode, _print_tree}, index::{Index, read_index}};

pub fn commit() -> () {
    let repo_tree = create_tree();
    extract_trees(repo_tree)
}

pub fn extract_trees(tree: IndexTreeNode) {
    
}

pub fn create_tree() -> IndexTreeNode {
    let index: Index = read_index();

    let mut root_node = IndexTreeNode {
        name: "root".to_string(),
        sha1_hash: "".to_string(),
        children: vec![]
    };
    
    for row in index.rows.iter() {
        let path_vec: Vec<&str> = row.file_name.split("/").collect();
        add_to_tree(&mut root_node, path_vec, row.blob_hash.to_string());
    }

    _print_tree(&root_node);

    root_node
}