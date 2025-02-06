pub fn sort_mut<T>(v: &mut Vec<T>) -> ()
where
    T: PartialOrd,
{
    let mut i = 0;
    if v.len() > 0 {
        while i < v.len() - 1 {
            let mut j = i + 1;
            let mut index = j;

            while j < v.len() {
                if v[j] < v[index] {
                    index = j
                }

                j += 1;
            }
            
            v.swap(i, index);
            i += 1;
        }
    }
}

#[cfg(test)]
mod sort_mut {
    use super::sort_mut;

    #[test]
    fn sort_mut_empty_vector() {
        let mut v: Vec<i32> = vec![];
        sort_mut(&mut v);

        assert_eq!(v, vec![]);
    }

    #[test]
    fn sort_mut_single_element_vector() {
        let mut v: Vec<i32> = vec![2];
        sort_mut(&mut v);

        assert_eq!(v, vec![2]);
    }

    #[test]
    fn sort_simple_vector() {
        let mut v = vec![10, 7, 9, 18, 9, 13];
        sort_mut(&mut v);

        assert_eq!(v, vec![7, 9, 9, 10, 13, 18]);
    }
}
