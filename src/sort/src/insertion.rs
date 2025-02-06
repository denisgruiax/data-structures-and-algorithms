fn sort_mut<T>(v: &mut Vec<T>) -> ()
where
    T: PartialOrd,
{
    let mut i = 1;

    while i < v.len() {
        let mut j = i;

        while j > 0 && v[j - 1] > v[j] {
            v.swap(j, j - 1);
            j -= 1;
        }

        i += 1;
    }
}

#[cfg(test)]
mod test_sort_mut {
    use super::sort_mut;

    #[test]
    fn sort_mut_empty_vector() {
        let mut v: Vec<i32> = vec![1];
        sort_mut(&mut v);

        assert_eq!(v, vec![1]);
    }
}
