fn sort<T>(v: &Vec<T>) -> Vec<T>
where
    T: PartialOrd + Clone,
{
    let mut acc = v.clone();
    let mut len = acc.len();

    if len == 0{
        return acc;
    }else {
        len-=1;
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

#[cfg(test)]
mod tests {
    use super::sort;
    use std::vec;
    #[test]
    fn sort_empty_vector() {
        let v: Vec<i32> = vec![];
        let v = sort(&v);

        assert_eq!(v, vec![]);
    }

    #[test]
    fn sort_simple_vector() {
        let v = vec![10, 7, 9, 18, 9, 13];
        let v = sort(&v);
        
        assert_eq!(v, vec![7, 9, 9, 10, 13, 18]);
    }
}
