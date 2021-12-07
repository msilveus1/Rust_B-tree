mod b_plus_node;
mod leaf;

struct b_plus_tree{
    number_of_children : i32,
    root_node : Node
}

pub impl b_plus_tree{
    pub fn new(first_value : String, number_of_children : i32 ){;
        b_plus_tree {
            root_node : Node::new(number_of_children)
            number_of_children : number_of_children
        }
    }

    pub fn add_value(&self, value : String) {
        let leaf = Leaf::new(value)
        if(!&self.root_node.has_children()){
            let number_leaves_in_root = root_node.get_leaves().len();
            &self.root_node.add_leaf(value);
            if(number_leaves_in_root + 1 == &self.number_of_children){
                let root_leaves = root_node.get_leaves();
                let split_leaf_value = root_leaves[leaves.len() + 1].get_value();
                let split_leaf_index = root_leaves.len() + 1;

                let new_child_node_left = Node::new(&self.number_of_children);
                let new_child_node_right = Node::new(&self.number_of_children);
                let new_root_node = Node::new(&self.number_of_children);
                let current_index = 0;
                new_child_node_left.add_leaves(root_leaves[0..split_leaf_index]);
                new_child_node_right.add_leaves(root_leaves[split_leaf_index..]);
                new_child_node_left.set_sibling_node(new_child_node_right)
                new_root_node.set_children_nodes(vec![new_child_node_left,new_child_node_right]);
                &self.root_node=new_root_node
            }
        }else {
            let current_node = root_node;        
            while(current_node.has_children()){

            }            
        }
    }

    

    fn split_node(){
            
    }



}