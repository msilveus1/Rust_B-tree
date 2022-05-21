use crate::b_plus_tree::BPlusNode::BPlusNode;

#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone)]
pub struct BPlusTree<T> {
    root_node : Option<BPlusNode<T>>,
    degree : usize,
}


impl<T : Ord + Clone> BPlusTree<T> {
    pub fn new(root_node : Option<BPlusNode<T>>, degree : usize) -> BPlusTree<T> {        
        if !root_node.is_none() {
            if root_node.clone().unwrap().get_node_leaves().len() >= degree {
                panic!("The given Root Node is too long for given degree")
            }
        }
        BPlusTree {
            root_node : root_node.clone(),
            degree : degree
        }
    }

    pub fn add_leaf(&mut self, leaf_value : T){
        let mut current_node = self.root_node.clone().unwrap();
        let mut current_node_leaves = self.root_node.clone().unwrap().get_node_leaves();
        while current_node.has_children() {
            let mut child_index = 0;
            for current_leaf in current_node_leaves.iter() {
                if leaf_value < *current_leaf {
                    break;
                } else {
                    child_index = child_index + 1;
                }
            }
            current_node = current_node.get_children_nodes()[child_index].clone();
            current_node_leaves = current_node.clone().get_node_leaves();
        }
        if (current_node_leaves.len() + 1) >= self.degree {
            

        }else {
            current_node.add_leaf(leaf_value);
        }
    }
}

#[cfg(test)]
mod tests { 
    use super::BPlusTree; 
    use crate::b_plus_tree::BPlusNode::BPlusNode;
    use std::panic;

    #[test]
    fn test_wrong_degree(){
        panic::set_hook(Box::new(|_info| {
            // do nothing
        }));    
        let test_root_node = BPlusNode::new(vec![443,43,43,4,4,3], None, None, Vec::new());
        let result = panic::catch_unwind(|| {
            let test_b_plus_tree = BPlusTree::new(Some(test_root_node.clone()),3);
        });
        assert!(result.is_err());
    }
    #[test]
    fn test(){
        println!("hello")
    }
}
