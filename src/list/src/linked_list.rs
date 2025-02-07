struct LinkedList<'a, T> {
    elt: &'a T,
    next: Option<Box<LinkedList<'a, T>>>,
}

impl<'a, T> LinkedList<'a, T> {
    pub fn new(elt: &'a T) -> LinkedList<'a, T> {
        LinkedList { elt, next: None }
    }
}

impl<'a, T> Iterator for LinkedList<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.elt;

        if let Some(next) = self.next.take() {
            *self = *next;
            Some(value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test_linked_list {
    use super::*;

    #[test]
    fn check() {
        let list = LinkedList::new(&1);

        assert_eq!(1, *list.elt);
        assert!(list.next.is_none());
    }
}
