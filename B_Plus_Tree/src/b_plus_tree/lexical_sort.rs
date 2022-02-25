use std::cmp::Ordering;
use std::ops::FnMut;

pub struct LexicalSort {
}

fn get_lexical_sort_refernce_vector() -> Vec<String> {
    let reference_vector =  vec![
        String::from("A"),
        String::from("B"),
        String::from("C"),
        String::from("D"),
        String::from("E"),
        String::from("F"),
        String::from("G"),
        String::from("H"),
        String::from("I"),
        String::from("J"),
        String::from("K"),
        String::from("L"),
        String::from("M"),
        String::from("N"),
        String::from("O"),
        String::from("P"),
        String::from("Q"),
        String::from("R"),
        String::from("S"),
        String::from("T"),
        String::from("U"),
        String::from("V"),
        String::from("W"),
        String::from("X"),
        String::from("Y"),
        String::from("Z"),
        String::from("a"),
        String::from("b"),
        String::from("c"),
        String::from("d"),
        String::from("e"),
        String::from("f"),
        String::from("g"),
        String::from("h"),
        String::from("i"),
        String::from("j"),
        String::from("k"),
        String::from("l"),
        String::from("m"),
        String::from("n"),
        String::from("o"),
        String::from("p"),
        String::from("q"),
        String::from("r"),
        String::from("s"),
        String::from("t"),
        String::from("u"),
        String::from("v"),
        String::from("w"),
        String::from("x"),
        String::from("y"),
        String::from("z"),
        String::from("1"),
        String::from("2"),
        String::from("3"),
        String::from("4"),
        String::from("5"),
        String::from("6"),
        String::from("7"),
        String::from("8"),
        String::from("9")
    ];
    reference_vector
}

fn find_string(input_char : String ) -> Option<usize> {
    let reference_vector = get_lexical_sort_refernce_vector();
    for (index, charecter) in reference_vector.iter().enumerate() {
        // println!("{}", *charecter);
        // println!("{}",(input_char == *charecter));
        if input_char == *charecter {
            return  Some(index);
        }
    }
    return None;
}


fn lexical_sort_function(string_a : &String, string_b : &String) -> Ordering {
    let lexical_sort_reference_vec = get_lexical_sort_refernce_vector();
    let mut current_char = 1;
    let mut return_value = Ordering::Equal;
    while(string_a.len() + 1 > current_char  || string_b.len() + 1 > current_char ) {
        let string_char_a = String::from(string_a.get(current_char-1..current_char).unwrap());
        let string_char_b = String::from(string_b.get(current_char-1..current_char).unwrap());
        let string_pos_a = find_string(string_char_a);
        let string_pos_b = find_string(string_char_b);
    
        
        if !string_pos_a.is_none() || !string_pos_b.is_none() {
            if string_pos_a.unwrap() > string_pos_b.unwrap() {
                return_value = Ordering::Greater;
                break;
            } else if string_pos_b.unwrap() > string_pos_a.unwrap() {
                return_value = Ordering::Less;
                break;
            }
        }else{
            panic!("Charecter was not found in sort") 
        }
        current_char = current_char + 1;
    }
    return_value
}

impl LexicalSort {
    pub fn get_lexical_sort() ->  impl FnMut(&String,&String) -> Ordering {
        lexical_sort_function
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lexical_sort() {
        let lexical_sort_fn_1 = LexicalSort::get_lexical_sort();
        let mut vec_1 = vec![String::from("Test2"),String::from("Test1")]; 
        vec_1.sort_by(lexical_sort_fn_1);
        assert_eq!(vec_1,vec![String::from("Test1"),String::from("Test2")]);
        let mut vec_2 = [String::from("bbb"),String::from("ddd"),String::from("aaa"),String::from("xxx")];
        let lexical_sort_fn_2 = LexicalSort::get_lexical_sort();
        vec_2.sort_by(lexical_sort_fn_2);
        assert_eq!(vec_2,[String::from("aaa"),String::from("bbb"),String::from("ddd"),String::from("xxx")]);
        let mut vec_3 = vec![String::from("bb"),String::from("dddddd"),String::from("a"),String::from("yyyy")];
        let lexical_sort_fn_3 = LexicalSort::get_lexical_sort();
        vec_3.sort_by(lexical_sort_fn_3);
        assert_eq!(vec_3,vec![String::from("a"),String::from("bb"),String::from("dddddd"),String::from("yyyy")]);
    }
}