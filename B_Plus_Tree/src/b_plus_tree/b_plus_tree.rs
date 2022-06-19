use crate::b_plus_tree::BPlusNode::BPlusNode;

#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone)]
pub struct BPlusTree<T> {
    root_node : Option<BPlusNode<T>>,
    degree : usize,
    depth : usize
}




impl<T : Ord + Clone> BPlusTree<T> {
    pub fn new(root_node : Option<BPlusNode<T>>, degree : usize) -> BPlusTree<T> {        
        if !root_node.is_none() {
            if root_node.clone().unwrap().get_node_leaves().len() >= degree {
                panic!("The given Root Node is too long for given degree")
            }
            else if root_node.clone().unwrap().has_parent() {
                panic!("The root node can not have a parent node")
            } 
        }
        
        BPlusTree {
            root_node : root_node.clone(),
            degree : degree,
            depth : 1
        }
    }
    
    fn split_node(node : BPlusNode<T>, degree : usize, is_lowest_level : bool) -> Vec<BPlusNode<T>> {
        let mut node_leaves = node.get_node_leaves();
        let split_index = degree/2;
        
        if is_lowest_level {
            let left_child_node_leaves = &node_leaves[..split_index];
            let right_child_node_leaves = &node_leaves[split_index..];
            return vec![
                BPlusNode::new(left_child_node_leaves.to_vec(),None,None,Vec::new()),
                BPlusNode::new(right_child_node_leaves.to_vec(),None,None,Vec::new()),
                BPlusNode::new(vec![node_leaves[split_index].clone()],None,None,Vec::new())
            ];
        } else {
            let left_child_node_leaves = &node_leaves[..split_index];
            let right_child_node_leaves = &node_leaves[split_index+1..];
            let new_parent_node_leaves = vec![node_leaves[split_index].clone()];
            return vec![
                BPlusNode::new(left_child_node_leaves.to_vec(),None,None,Vec::new()),
                BPlusNode::new(right_child_node_leaves.to_vec(),None,None,Vec::new()),
                BPlusNode::new(new_parent_node_leaves,None,None,Vec::new()),
            ]        
        }
    }

    fn get_tree_levels(&self) -> Vec<Vec<BPlusNode<T>>> {
        let mut level = 1;
        let mut tree_levels = Vec::new();
        let mut current_nodes = vec![self.root_node.clone().unwrap()];
        let mut next_level = Vec::new();
        while level != self.depth + 1 {
            for node in current_nodes.clone() {
                next_level.append(&mut node.get_children_nodes().clone());
            }
            tree_levels.push(current_nodes.clone());
            current_nodes = next_level.clone();
            level = level + 1;
        }
        tree_levels
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
            let current_level = self.depth;
            let mut current_level_split = true;
            let mut current_split_nodes = Vec::new();
            let mut current_leaf_value = leaf_value.clone();
            while current_level_split {
                current_node.add_leaf(current_leaf_value.clone());
                current_split_nodes = BPlusTree::<T>::split_node(current_node.clone(), self.degree, current_level == self.depth);
                // Linking the bottom leaves
                let mut left_child_node = current_split_nodes[0].clone();
                let right_child_node = current_split_nodes[1].clone();
                if  current_level == self.depth {
                    left_child_node.add_sibling(right_child_node.clone());
                    //TODO: Finish this with reverse node implementation
                }  
                let mut parent_node = current_node.get_parent_node().unwrap();
                parent_node.delete_child_node(current_node.clone());
                parent_node.add_child_node(left_child_node.clone());
                parent_node.add_child_node(right_child_node.clone());
                current_leaf_value = current_split_nodes[2].get_node_leaves()[0].clone();
                current_node = parent_node;
                current_level_split = current_node.get_node_leaves().len() + 1 >= self.degree;
            }
            current_node.add_leaf(current_leaf_value.clone());
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
    fn test_root_node_no_parrent(){
        panic::set_hook(Box::new(|_info| {
            // do nothing
        }));    
        let test_parent_node = BPlusNode::new(vec![1], None, None, Vec::new());
        let test_root_node = BPlusNode::new(vec![443,43,43,4,4,3], None, Some(test_parent_node), Vec::new());
        let result = panic::catch_unwind(|| {
            let test_b_plus_tree = BPlusTree::new(Some(test_root_node.clone()),3);
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_adding_leaf_under_degree(){
        let mut test_root_node = BPlusNode::new(vec![3], None, None, Vec::new());
        let mut test_b_plus_tree = BPlusTree::new(Some(test_root_node.clone()),3);
        let test_leaf_1 = 5;
        test_b_plus_tree.add_leaf(test_leaf_1.clone());
        let test_tree_levels = test_b_plus_tree.get_tree_levels();
        println!("{:?}",test_tree_levels);

        
    }
}
