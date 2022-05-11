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
        let test_root_node = BPlusNode::new(vec![443,43,43,4,4,3], None, Vec::new());
        let result = panic::catch_unwind(|| {
            let test_b_plus_tree = BPlusTree::new(Some(test_root_node.clone()),3);
        });
        assert!(result.is_err());
    }
}
