pub fn is_sorted(vec: Vec<i64>) -> bool {
    for i in 0..vec.len() - 1 {
        if vec[i] > vec[i + 1] {
            return false;
        }
    }
    true
}

pub fn is_equal_vec(vec1: Vec<i64>, vec2: Vec<i64>) -> bool {
    if vec1.len() != vec2.len() {
        return false;
    }
    for i in 0..vec1.len() {
        if vec1[i] != vec2[i] {
            return false;
        }
    }
    true
}

pub fn binary_search(input: &[i64], key: i64) -> usize {
    let (mut low, mut high) = (0, input.len());

    while low < high {
        let mid = (low + high) / 2;
        if key <= input[mid] {
            high = mid
        } else {
            low = mid + 1
        }
    }
    high
}

#[cfg(test)]
mod tests {
    use crate::utils::binary_search;

    #[test]
    fn binary_search_element_missing() {
        let input: Vec<i64> = vec![1, 2, 3, 5, 6, 7, 8, 9];
        let index = binary_search(&*input, 4);
        assert_eq!(Err(index), input.binary_search(&4));
    }

    #[test]
    fn binary_search_middle() {
        let input: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let index = binary_search(&*input, 4);
        assert_eq!(Ok(index), input.binary_search(&4));
    }

    #[test]
    fn binary_search_right_edge() {
        let input: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let index = binary_search(&*input, 9);
        assert_eq!(Ok(index), input.binary_search(&9));
    }

    #[test]
    fn binary_search_right_edge_missing() {
        let input: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let index = binary_search(&*input, 9);
        assert_eq!(Err(index), input.binary_search(&9));
    }

    #[test]
    fn binary_search_left_edge() {
        let input: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let index = binary_search(&*input, 1);
        assert_eq!(Ok(index), input.binary_search(&1));
    }

    #[test]
    fn binary_search_left_edge_missing() {
        let input: Vec<i64> = vec![2, 3, 4, 5, 6, 7, 8, 9];
        let index = binary_search(&*input, 1);
        assert_eq!(Err(index), input.binary_search(&1));
    }
}
