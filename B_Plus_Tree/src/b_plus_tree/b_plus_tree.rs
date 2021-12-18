mod b_plus_node;
mod leaf;
use lexical_sort::natural_lexical_cmp;

struct b_plus_tree{
    number_of_children : i32,
    split_index : i32
    root_node : Node
}

pub impl b_plus_tree{
    pub fn new(first_value : String, number_of_children : i32 ){;
        b_plus_tree {
            root_node : Node::new(number_of_children,None),
            number_of_children : number_of_children,
            split_index : (number_of_children/2 + number_of_children%2)
        }
    }

    pub fn add_value(&self, value : String) {
        let leaf = Leaf::new(value)
        if(!&self.root_node.has_children()){
            let number_leaves_in_root = root_node.get_leaves().len();
            &self.root_node.add_leaf(value);
            if(number_leaves_in_root + 1 == &self.number_of_children){
                let root_leaves = root_node.get_leaves();
                let split_leaf_value = root_leaves[&self.split_index].get_value();
                let split_leaf_index = root_leaves.len() + 1;

                let new_root_node = Node::new(&self.number_of_children);
                let new_child_node_left = Node::new(&self.number_of_children, Some(new_root_node));
                let new_child_node_right = Node::new(&self.number_of_children, Some(new_root_node));
                new_child_node_left.add_leaves(root_leaves[0..&self.split_leaf_index]);
                new_child_node_right.add_leaves(root_leaves[&self.split_leaf_index..]);
                new_child_node_left.set_sibling_node(new_child_node_right);
                new_child_node_right.set_reverse_sibling(new_child_node_left);
                new_root_node.set_children_nodes(vec![new_child_node_left,new_child_node_right]);
                &self.root_node=new_root_node
            }
        }else {
            let mut current_node = &self.root_node;        
            while(current_node.has_children()){
                let child_index = get_child_index(value,current_node.get_flattened_leaves());
                current_node = child_node[child_index];
            }

            if(current_node.get_leaves().len() + 1 == &self.number_of_children ){
                while(current_node.has_parent()){
                    current_node.add_leaf(value);
                    let split_leaf_value = current_node.get_leaves()[&self.split_leaf_index].get_value();
                    let current_leaves = current_node.get_leaves();
                    
                    let new_child_node_left = Node::new(&self.number_of_children, Some(current_node.get_parent_node()));
                    let new_chlid_node_right = Node::new(&self.number_of_children, Some(current_node.get_parent_node()));

                    new_child_node_left.add_leaves(current_leaves[0..&self.split_leaf_index]);
                    new_child_node_right.add_leaves(current_leaves[&self.split_leaf_index..]);
                    
                    if(current_node.has_children()){
                        let old_node_children = current_node.get_children_nodes();
                        new_child_node_left.set_children_nodes(old_node_children[0..&self.split_leaf_index]);
                        new_child_node_right.set_children_nodes(old_node_children[&self.split_leaf_index..]);
                        new_child_node_left.reset_children_parents();
                        new_child_node_right.reset_children_parents();   
                    }else{
                        new_child_node_right.set_sibling_node(current_node.get_sibbling());
                        new_child_node_left.set_sibling_node(new_child_node_right);
                        new_child_node_right.set_reverse_sibling(new_child_node_left);
                        let current_reverse_sibling = current_node.get_reverse_sibling();
                        current_reverse_sibling.set_sibling_node(new_child_node_left);
                        new_child_node_left.set_reverse_sibling(current_reverse_sibling);
                    }

                    if(current_node.get_parent_node().get_number_of_leaves() + 1 != &self.number_of_children){
                        current_node.get_parent_node().add_leaf(split_leaf_value);
                        break;
                    }
                    current_node = current_node.get_parent_node();

                }
            }
        }
    }



    fn get_child_index(value : String, flattened_leaves : Vec<String>) -> i32 {
        let index_found = false;
        current_index = 0;
        while(!index_found){
            let cross_section=vec![value,flattened_leaves[current_index]]
            let sorted_vector = cross_section.clone().sort_by(natural_lexical_cmp)
            if(sorted_vector != cross_section){
                if(current_index == flattened_leaves.len()){
                    index_found=true;
                }
                current_index++;
            }else{
                index_found=true;
            }
        }
        current_index
    }
}