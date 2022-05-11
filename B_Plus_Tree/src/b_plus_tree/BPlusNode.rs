
#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone)]
pub struct BPlusNode<T> {
    sibling_node : Option<Box<BPlusNode<T>>>,
    node_leaves : Vec<T>,
    children_nodes : Vec<BPlusNode<T>>
}


impl<T : Ord + Clone> BPlusNode<T> {
    pub fn new(mut node_leaves : Vec<T>, sibling_node : Option<BPlusNode<T>>, children_nodes : Vec<BPlusNode<T>>) -> BPlusNode<T> { 
        node_leaves.sort();
        if !sibling_node.is_none() {            
            let unwrapped_node = sibling_node.unwrap();
            BPlusNode {
                sibling_node : Some(Box::new(unwrapped_node)),
                node_leaves : node_leaves,
                children_nodes : children_nodes
            }
        } else {
            BPlusNode {
                sibling_node : None, 
                node_leaves : node_leaves,
                children_nodes : children_nodes
            }
        }

    }

    pub fn get_sibling_node(&self) -> Option<BPlusNode<T>> {
        if self.sibling_node.is_none() {
            return None;
        }else {
            let unwrapped_node = *(self.sibling_node.clone().unwrap());
            Some(unwrapped_node)
        }
    }
    
    pub fn get_node_leaves(&self) -> Vec<T> {
        self.node_leaves.clone()
    }

    pub fn get_children_nodes(&self) -> Vec<BPlusNode<T>> {
        self.children_nodes.clone()
    }

    pub fn add_leaf(&mut self, leaf_value : T) {
        self.node_leaves.push(leaf_value);
        self.node_leaves.sort();
    }

    pub fn add_child_node(&mut self, child_node : BPlusNode<T>){
        self.children_nodes.push(child_node);
        self.children_nodes.sort();
    }
}


#[cfg(test)]
mod tests { 
    use super::BPlusNode;
    #[test]
    fn test_object_creation(){
        let test_leaf_vector_1 = vec![1,2,3,4,5,6];
        let test_b_plus_node = BPlusNode::new(test_leaf_vector_1.clone(),None,Vec::new());
        assert_eq!(test_leaf_vector_1,test_b_plus_node.get_node_leaves());        
    }
    
    #[test]
    fn test_sibling_node() {
        let sibling_node = BPlusNode::new(vec![1,2,3,4,5],None,Vec::new());
        let main_test_node_1 = BPlusNode::new(vec![1,3,4,5],Some(sibling_node.clone()),Vec::new());
        assert_eq!(sibling_node.clone(),main_test_node_1.get_sibling_node().unwrap());

        let main_test_node_2 = BPlusNode::new(vec![1,2],None,Vec::new());
        assert_eq!(main_test_node_2.get_sibling_node().is_none(),true);
    }
    #[test]
    fn test_node_leaves(){
        let unsorted_vec = vec![1,5,2,4,3];
        let sorted_vec = vec![1,2,3,4,5];
        let main_test_node_1 = BPlusNode::new(unsorted_vec.clone(), None, Vec::new());
        assert_eq!(sorted_vec, main_test_node_1.get_node_leaves())
    }

    #[test]
    fn test_children_nodes(){
        let child_node_1 = BPlusNode::new(vec![1,2], None, Vec::new());
        let child_node_2 = BPlusNode::new(vec![3,4], None, Vec::new());
        let child_node_vec = vec![child_node_1,child_node_2];
        let parent_node = BPlusNode::new(vec![8,3], None, child_node_vec.clone());
        assert_eq!(child_node_vec,parent_node.get_children_nodes());
    }

    #[test] 
    fn test_adding_leaf(){
        let final_leaf_vec = vec![1,2,3,4,5,6];
        let initial_leaf_vec = vec![1,2,4,5];
        let mut test_node = BPlusNode::new(initial_leaf_vec.clone(), None, Vec::new());
        test_node.add_leaf(3);
        test_node.add_leaf(6);
        assert_eq!(test_node.get_node_leaves(),final_leaf_vec);
    }
    #[test]
    fn test_adding_children(){
        let mut test_node = BPlusNode::new(vec![1,2,3,4,5], None, Vec::new());
        let child_node = BPlusNode::new(vec![1,3,21,15,22,34], None, Vec::new());
        test_node.add_child_node(child_node.clone());
        assert_eq!(test_node.get_children_nodes(),vec![child_node.clone()]);
    }
}