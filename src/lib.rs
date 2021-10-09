pub fn sort<T: Eq + Ord + Clone>(a: &mut Vec<T>) {
    let end = a.len();
    for i in 0..end {
        for j in 0..end {
            if a[i] < a[j] {
                a.swap(i, j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_int() {
        let mut vector = vec![3, 1, 6, 7, 4, 2];
        sort(&mut vector);
        assert_eq!(vec![1, 2, 3, 4, 6, 7], vector);
    }

    #[test]
    fn test_sort_str() {
        let mut vector = vec!["c", "a", "b", "d", "e"];
        sort(&mut vector);
        assert_eq!(vec!["a", "b", "c", "d", "e"], vector);
    }
}
