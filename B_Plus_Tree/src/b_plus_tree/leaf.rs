#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Leaf{
    value : String
}

impl Leaf{
    pub fn new(value : String) -> Leaf{
        Leaf {
            value: value
        }
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }   
}