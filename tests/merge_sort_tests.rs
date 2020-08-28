#[cfg(test)]
mod merge_sort {
    use sort_it::merge_sort;

    #[test]
    fn merge_sort_int() {
        let sorted_list = merge_sort(vec![1, 32, 53, 13, 7]);
        assert_eq!(sorted_list, vec![1, 7, 13, 32, 53]);
    }

    #[test]
    fn merge_sort_float() {
        let sorted_list = merge_sort(vec![2.0, 0.3, 0.1, 0.6]);
        assert_eq!(sorted_list, vec![0.1, 0.3, 0.6, 2.0]);
    }

    #[test]
    fn merge_sort_char() {
        let sorted_list = merge_sort(vec!['b', 'd', 'a', 'c']);
        assert_eq!(sorted_list, vec!['a', 'b', 'c', 'd']);
    }

    #[test]
    fn merge_sort_str() {
        let sorted_list = merge_sort(vec!["bac", "cba", "abc"]);
        assert_eq!(sorted_list, vec!["abc", "bac", "cba"]);
    }
}

