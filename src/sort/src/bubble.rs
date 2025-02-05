fn sort<T>(v: &Vec<T>) -> Vec<T>
where
    T: PartialOrd + Clone,
{
    let mut acc = v.clone();
    let mut len = acc.len();

    if len == 0 {
        return acc;
    } else {
        len -= 1;
    }

    for _ in 0..len {
        for j in 0..len {
            match acc[j] > acc[j + 1] {
                true => acc.swap(j, j + 1),
                false => (),
            }
        }
    }

    acc
}

fn sort_mut<T>(v: &mut Vec<T>) -> ()
where
    T: PartialOrd,
{
    let mut len = v.len();

    if len != 0 {
        len -= 1;
    }

    for _ in 0..len {
        for j in 0..len {
            match v[j] > v[j + 1] {
                true => v.swap(j, j + 1),
                false => (),
            }
        }
    }
}

#[cfg(test)]
mod test_sort {
    use super::sort;
    use std::vec;
    #[test]
    fn sort_empty_vector() {
        let v: Vec<i32> = vec![];
        let v = sort(&v);

        assert_eq!(v, vec![]);
    }

    #[test]
    fn sort_single_element_vector() {
        let v: Vec<i32> = vec![1];
        let v = sort(&v);

        assert_eq!(v, vec![1]);
    }

    #[test]
    fn sort_simple_vector() {
        let v = vec![10, 7, 9, 18, 9, 13];
        let v = sort(&v);

        assert_eq!(v, vec![7, 9, 9, 10, 13, 18]);
    }
}

mod test_sort_mut{
    use super::sort_mut;
    use std::vec;

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
}