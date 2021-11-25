mod leaf;

struct Node{
    number_of_children : i32,
    leaves : Vec<leaf>,
    children_nodes: Vec<Node>,
    linked_sibling: Node
}

impl Node{
    pub fn new( number_of_children : i32) -> {
        Node {
            number_of_children : number_of_children,
            leaves : Vec::new(),
            children_nodes: Vec::new(),
            linked_sibling : None
        }
    }
}