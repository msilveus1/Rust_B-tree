mod leaf;

struct Node{
    number_of_children : i32,
    leaves : Vec<Leaf>,
    children_nodes: Vec<Node>,
    linked_sibling: Node,
}

pub impl Node{
    pub fn new( number_of_children : i32) -> {
        Node {
            number_of_children : number_of_children,
            leaves : Vec::new(),
            children_nodes: Vec::new(),
            linked_sibling : None
        }
    }

    pub fn add_sibling(&self, sibling_node : Node) -> {
        &self.linked_sibling = sibling_node;
    }
    
    pub fn get_number_of_leaves(&self) => i32 {
        &self.leaves.len()
    }
    
    pub fn get_sibbling(&self) -> Node {
        &self.linked_sibling
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

    pub fn add_leaves(&self, leaves : Vec<Leaf>) -> {
        &self.leaves = leaves;
    }

    pub fn add_leaf(&self, leaf : Leaf) -> {
        // Have to figure out a good sorting algorithm here for sorting
    }
    
    pub fn set_sibling_node(&self, sibling_node : Node) {
        &self.linked_sibling=sibling_node;
    }

    pub fn set_children_nodes(&self, children_nodes : Vec<Node>){
        &self.children_nodes = children_nodes;
    }


}