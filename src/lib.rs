#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    /* unimplemented!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list."); */
    if _first_list == _second_list {
        Comparison::Equal
    } else if is_sublist(_first_list, _second_list) {
        Comparison::Sublist
    } else if is_sublist(_second_list, _first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }

}

fn is_sublist<T: PartialEq>(smaller: &[T], larger: &[T]) -> bool {
    match smaller.len() {
        0 => true,
        len if len > larger.len() => false,
        _ => {
            for i in 0..=(larger.len() - smaller.len()) {
                if &larger[i..i + smaller.len()] == smaller {
                    return true;
                }
            }
            false
        }
    }
}