
#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone)]
pub struct BPlusNode<T> {
    parent_node : Option<Box<BPlusNode<T>>>,
    sibling_node : Option<Box<BPlusNode<T>>>,
    node_leaves : Vec<T>,
    children_nodes : Vec<BPlusNode<T>>
}


impl<T : Ord + Clone> BPlusNode<T> {
    pub fn new(mut node_leaves : Vec<T>, sibling_node : Option<BPlusNode<T>>, parent_node : Option<BPlusNode<T>>, children_nodes : Vec<BPlusNode<T>>) -> BPlusNode<T> { 
        node_leaves.sort();
        let mut unwrapped_sibling_node = None;
        let mut unwrapped_parent_node = None;
        
        if !sibling_node.is_none() {            
            unwrapped_sibling_node = Some(Box::new(sibling_node.unwrap()));
        }
        
        if !parent_node.is_none() {
            unwrapped_parent_node = Some(Box::new(parent_node.unwrap()));
        }
        
        BPlusNode {
            parent_node: unwrapped_parent_node,
            sibling_node : unwrapped_sibling_node,
            node_leaves : node_leaves,
            children_nodes : children_nodes
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

    pub fn get_parent_node(&self) -> Option<BPlusNode<T>> {
        if self.parent_node.is_none() {
            return None;
        }else {
            let unwrapped_node = *(self.parent_node.clone().unwrap());
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

    pub fn add_sibling(&mut self, sibling : BPlusNode<T>) {
        self.sibling_node=Some(Box::new(sibling.clone()))
    }

    pub fn add_child_node(&mut self, child_node : BPlusNode<T>){
        self.children_nodes.push(child_node);
        self.children_nodes.sort();
    }

    pub fn has_children(&self) -> bool {
        !(self.children_nodes.len() == 0)
    }
    pub fn has_sibling(&self) -> bool {
        !(self.sibling_node.is_none())
    }

    pub fn has_parent(&self) -> bool {
        !(self.parent_node.is_none())
    }

    pub fn delete_child_node(&mut self, delete_node : BPlusNode<T>) {
        let mut index = 0;
        for child in self.children_nodes.clone() {
            if delete_node == child {
                self.children_nodes.remove(index);
                break;
            } 
        }
    }

}


#[cfg(test)]
mod tests { 
    use super::BPlusNode;
    #[test]
    fn test_object_creation(){
        let test_leaf_vector_1 = vec![1,2,3,4,5,6];
        let test_b_plus_node = BPlusNode::new(test_leaf_vector_1.clone(),None,None,Vec::new());
        assert_eq!(test_leaf_vector_1,test_b_plus_node.get_node_leaves());        
    }
    
    #[test]
    fn test_sibling_node() {
        let sibling_node = BPlusNode::new(vec![1,2,3,4,5],None,None,Vec::new());
        let main_test_node_1 = BPlusNode::new(vec![1,3,4,5],Some(sibling_node.clone()),None,Vec::new());
        assert_eq!(sibling_node.clone(),main_test_node_1.get_sibling_node().unwrap());

        let main_test_node_2 = BPlusNode::new(vec![1,2], None, None, Vec::new());
        assert_eq!(main_test_node_2.get_sibling_node().is_none(),true);
    }
    #[test]
    fn test_node_leaves(){
        let unsorted_vec = vec![1,5,2,4,3];
        let sorted_vec = vec![1,2,3,4,5];
        let main_test_node_1 = BPlusNode::new(unsorted_vec.clone(), None, None, Vec::new());
        assert_eq!(sorted_vec, main_test_node_1.get_node_leaves())
    }

    #[test]
    fn test_children_nodes(){
        let child_node_1 = BPlusNode::new(vec![1,2], None, None, Vec::new());
        let child_node_2 = BPlusNode::new(vec![3,4], None, None, Vec::new());
        let child_node_vec = vec![child_node_1,child_node_2];
        let parent_node = BPlusNode::new(vec![8,3], None, None, child_node_vec.clone());
        assert_eq!(child_node_vec,parent_node.get_children_nodes());
    }

    #[test] 
    fn test_adding_leaf(){
        let final_leaf_vec = vec![1,2,3,4,5,6];
        let initial_leaf_vec = vec![1,2,4,5];
        let mut test_node = BPlusNode::new(initial_leaf_vec.clone(), None, None, Vec::new());
        test_node.add_leaf(3);
        test_node.add_leaf(6);
        assert_eq!(test_node.get_node_leaves(),final_leaf_vec);
    }
    #[test]
    fn test_adding_children(){
        let mut test_node = BPlusNode::new(vec![1,2,3,4,5], None, None, Vec::new());
        let child_node = BPlusNode::new(vec![1,3,21,15,22,34], None, None, Vec::new());
        test_node.add_child_node(child_node.clone());
        assert_eq!(test_node.get_children_nodes(),vec![child_node.clone()]);
    }
    #[test]
    fn test_has_children(){
        let mut test_node = BPlusNode::new(vec![1,2,3,4,5], None, None, Vec::new());
        let child_node = BPlusNode::new(vec![1,3,21,15,22,34], None, None, Vec::new());
        assert!(!test_node.has_children());
        test_node.add_child_node(child_node.clone());
        assert!(test_node.has_children());
    }
    #[test]
    fn test_has_sibling() {
        let mut test_node = BPlusNode::new(vec![1,2,3,4,5], None, None, Vec::new());
        let sibling_node = BPlusNode::new(vec![1,3,21,15,22,34], None, None, Vec::new());
        assert!(!test_node.has_sibling());
        test_node.add_sibling(sibling_node);
        assert!(test_node.has_sibling());
    }

    #[test]
    fn test_parent_node() {
        let mut test_parent_node = BPlusNode::new(vec![1,2,3,4,5], None, None, Vec::new());
        let mut test_node = BPlusNode::new(vec![1,2,3,4,5],None, Some(test_parent_node.clone()),Vec::new());
        assert_eq!(test_parent_node.clone(),test_node.get_parent_node().unwrap());
        assert!(test_node.has_parent());
    }

    #[test]
    fn delete_child_node() {
        let mut test_parent_node = BPlusNode::new(vec![1,2,3,4,5], None, None, Vec::new());
        let mut test_node = BPlusNode::new(vec![1,2,3,4,5],None, Some(test_parent_node.clone()),Vec::new());
        test_parent_node.add_child_node(test_node.clone());
        test_parent_node.delete_child_node(test_node.clone());
        assert_eq!(test_parent_node.get_children_nodes().len(),0);
    }   

}