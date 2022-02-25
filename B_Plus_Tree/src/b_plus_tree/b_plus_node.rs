
use crate::b_plus_tree::leaf::Leaf;
use crate::b_plus_tree::lexical_sort::LexicalSort;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Node{
    number_of_children : usize,
    leaves : Vec<Leaf>,
    children_nodes: Vec<Node>,
    parent_node : Option<Box<Node>>,
    linked_sibling: Option<Box<Node>>,
    reverse_sibling: Option<Box<Node>>
}


impl Node{
    pub fn new( number_of_children : usize, parent_node : Option<Node>) -> Node{
        if !parent_node.is_none() {
            Node {
                number_of_children : number_of_children,
                leaves : Vec::new(),
                children_nodes : Vec::new(),
                parent_node : Some(Box::new(parent_node.unwrap())),
                linked_sibling : None,
                reverse_sibling : None
            }
        }else {
            Node {
                number_of_children : number_of_children,
                leaves : Vec::new(),
                children_nodes : Vec::new(),
                parent_node : None,
                linked_sibling : None,
                reverse_sibling : None
            }
        }
    }

    pub fn add_leaf(&mut self, leaf : Leaf)  {
        let mut flat_leaf_vector_copy : Vec<String> = Vec::new();
        for leaf in self.leaves.iter() {
            flat_leaf_vector_copy.push(leaf.get_value());
        }
        flat_leaf_vector_copy.push(leaf.get_value());
        flat_leaf_vector_copy.sort_by(LexicalSort::get_lexical_sort());

        let mut new_leaf_vector = Vec::new();

        for flat_leaf in flat_leaf_vector_copy.iter() {
            
            new_leaf_vector.push(Leaf::new(flat_leaf.clone()))
        }
        
        self.leaves = new_leaf_vector;
    }    
    
    pub fn set_sibling_node(&mut self, sibling_node : Node) {
        self.linked_sibling=Some(Box::new(sibling_node));
    }
    
    pub fn get_sibbling_node(&self) -> Node {
        *(self.linked_sibling.clone().unwrap())
    }

    pub fn add_children(&mut self, children : Vec<Node>) {
        if self.children_nodes.is_empty() {
            self.children_nodes = children;
        } else {
            let mut temp_child_vec = self.children_nodes.clone();
            temp_child_vec.extend(children.clone());
            self.children_nodes = temp_child_vec;
        }
    }

    pub fn get_children(&mut self) -> Vec<Node> {
        self.children_nodes.clone()
    }

    pub fn get_flat_leaves(&self) -> Vec<String> {
        let mut flat_leaf_vector : Vec<String> = Vec::new();
        for leaf in self.leaves.iter() {
            flat_leaf_vector.push(leaf.get_value());
        }
        flat_leaf_vector
    }
    
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn testing_sort() {
        let mut test_node_1 = Node::new(3,None);
        let mut test_node_2 = Node::new(3,None);
        let test_leaf_1 = Leaf::new(String::from("abcd"));
        let test_leaf_2 = Leaf::new(String::from("wxyz"));
        let expected_leaf_order_1 = vec![String::from("abcd"),String::from("wxyz")];

        test_node_1.add_leaf(test_leaf_1);
        assert_eq!(test_node_1.leaves.len(),1);

        test_node_1.add_leaf(test_leaf_2);
        assert_eq!(test_node_1.leaves.len(),2);
        assert_eq!(expected_leaf_order_1, test_node_1.get_flat_leaves());

        let test_leaf_3 = Leaf::new(String::from("abcd"));
        let test_leaf_4 = Leaf::new(String::from("wxyz"));
        test_node_2.add_leaf(test_leaf_4);
        test_node_2.add_leaf(test_leaf_3);
        assert_eq!(expected_leaf_order_1, test_node_2.get_flat_leaves());


        let mut test_node_3 = Node::new(3,None);
        let test_leaf_5 = Leaf::new(String::from("aaaa"));
        let test_leaf_6 = Leaf::new(String::from("bbbb"));
        let test_leaf_7 = Leaf::new(String::from("cccc"));
        let expected_leaf_order_2 = vec![String::from("aaaa"),String::from("bbbb"),String::from("cccc")];

        test_node_3.add_leaf(test_leaf_7);
        test_node_3.add_leaf(test_leaf_6);
        test_node_3.add_leaf(test_leaf_5);

        assert_eq!(expected_leaf_order_2, test_node_3.get_flat_leaves());
        
        let mut test_node_4 = Node::new(3,None);
        let test_leaf_8 = Leaf::new(String::from("11"));
        let test_leaf_9 = Leaf::new(String::from("12"));
        let test_leaf_10 = Leaf::new(String::from("aa"));
        let expected_leaf_order_3 = vec![String::from("aa"),String::from("11"),String::from("12")];

        test_node_4.add_leaf(test_leaf_10);
        test_node_4.add_leaf(test_leaf_9);
        test_node_4.add_leaf(test_leaf_8);

        assert_eq!(expected_leaf_order_3,test_node_4.get_flat_leaves());

    }

    #[test]
    fn testing_siblings() {
        let mut test_sibbling_node_1 = Node::new(3,None);
        let test_leaf_1 = Leaf::new(String::from("1"));
        test_sibbling_node_1.add_leaf(test_leaf_1);
        
        let mut test_sibbling_node_2 = Node::new(3,None);
        let test_leaf_2 = Leaf::new(String::from("2"));
        test_sibbling_node_2.add_leaf(test_leaf_2);
        test_sibbling_node_1.set_sibling_node(test_sibbling_node_2.clone());
        assert_eq!(test_sibbling_node_2.get_flat_leaves(), test_sibbling_node_1.get_sibbling_node().get_flat_leaves());

    }

    #[test]
    fn testing_children_node(){
        let mut test_parent_node_1 = Node::new(3,None);
        let mut index = 0;
        let mut test_children_nodes = Vec::new();
        while index < 3 {
            let mut indexed_node = Node::new(3,None);
            indexed_node.add_leaf(Leaf::new(String::from(index.to_string())));
            test_children_nodes.push(indexed_node);
            index = index + 1;
        }
        test_parent_node_1.add_children(test_children_nodes.clone());        
        assert_eq!(test_children_nodes,test_parent_node_1.get_children());

        let mut test_parent_node_2 = Node::new(3,None);
        let mut test_children_nodes_2 = vec![Node::new(5,None),Node::new(4,None)];
        let mut test_children_nodes_3 = vec![Node::new(4,None),Node::new(3,None)];

        test_parent_node_2.add_children(test_children_nodes_2.clone());
        test_parent_node_2.add_children(test_children_nodes_3.clone());
        let mut composite_test_children_nodes = test_children_nodes_2.extend(test_children_nodes_3.clone());
    }
}


