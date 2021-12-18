
mod leaf;
use lexical_sort::natural_lexical_cmp;

struct Node{
    number_of_children : i32,
    leaves : Vec<Leaf>,
    children_nodes: Vec<Node>,
    parent_node : Option<Node>,
    linked_sibling: Option<Node>,
}

pub impl Node{
    pub fn new( number_of_children : i32, parent_node : Option<Node>) -> {
        Node {
            number_of_children : number_of_children,
            leaves : Vec::new(),
            children_nodes : Vec::new(),
            parent_node : parent_node,
            linked_sibling : None
        }
    }

    pub fn add_sibling(&self, sibling_node : Node) -> {
        &self.linked_sibling = Some(sibling_node);
    }
    
    pub fn get_number_of_leaves(&self) => i32 {
        &self.leaves.len()
    }
    
    pub fn get_sibbling(&self) -> Node {
        &self.linked_sibling.unwrap()
    }

    pub fn get_number_of_children(&self) -> i32{
        &self.number_of_children
    }

    pub fn get_leaves(&self) -> Vec<Leaf> {
        &self.leaves
    }

    pub fn get_children_nodes(&self){
        &self.children_nodes
    }

    pub fn get_child_by_index(&self, index : i32) -> Node {
        &self.children_nodes[index]
    }

    pub fn get_parent_node(&self) -> Node {
        &self.parent_node.unwrap()
    }

    pub fn add_leaves(&self, leaves : Vec<Leaf>) -> {
        &self.leaves = leaves;
    }

    pub fn add_leaf(&self, leaf : Leaf) -> {
        // Have to figure out a good sorting algorithm here for sorting
        let flat_leaf_list=flatten_node(&self.leaves);
        flat_leaf_list.sort_by(natural_lexical_cmp)
        //TODO: improve this algorithm here because this is really bad.
        new_leaf_structure = Vec::new();
        for flat_leaf in flat_leaf_list.iter() {
            new_leaf_structure.push(Leaf::new(flat_leaf));
        }
        &self.leaves = new_leaf_structure;
    }
    
    pub fn set_sibling_node(&self, sibling_node : Node) {
        &self.linked_sibling=sibling_node;
    }

    pub fn set_children_nodes(&self, children_nodes : Vec<Node>){
        &self.children_nodes = children_nodes;
    }

    pub fn has_children(&self) : bool {
        &self.children_nodes.len() > 0
    }

    pub fn has_parent(&self) : bool {
        let has_parent = false;
        match &self.parent_node {
            None => {  } //Do Nothing
            Some => has_parent = true
        }
        has_parent
    }

    pub fn get_flattened_leaves(&self) -> Vec<String> {
        flatten_leaves(&self.leaves)
    }

    fn flatten_leaves(leaves : Vec<Leaf>) -> Vec<String> {
        let flattened_leaves = Vec::new();
        for leaf in leaves.iter() {
            flattened_leaves.push(leaf.get_value());
        }
        flattened_leaves
    }
}