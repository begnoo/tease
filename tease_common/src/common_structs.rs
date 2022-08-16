pub struct TreeNode {
    pub sha1: String,
    pub path: String,
    pub node_type: String,
    pub node_content: String,
    pub children: Vec<TreeNode>,
}

pub struct Commit {
    pub sha1: String,
    pub parent: String,
    pub author: String,
    pub children: Vec<TreeNode>,
}


// - read tree -> json {name, path, children: [{name, type}]} []
// - read blob -> json {name, path, content, type} []
// - read commit -> slicno kao tree ?? []