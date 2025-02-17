struct LinkedList<'a, T> {
    elt: &'a T,
    len: usize,
    next: Option<Box<LinkedList<'a, T>>>,
}

impl<'a, T> LinkedList<'a, T> {
    pub fn new(elt: &'a T) -> LinkedList<'a, T> {
        LinkedList {
            elt,
            len: 1,
            next: None,
        }
    }

    pub fn push(&mut self, elt: &'a T) -> () {
        let mut tail = self;

        while let Some(ref mut next_node) = tail.next {
            tail = next_node;
        }

        tail.next = Some(Box::new(LinkedList {
            elt,
            len: tail.len + 1,
            next: None,
        }));
    }

    pub fn get(&mut self, pos: usize) -> Option<&'a T> {
        let mut list = self;

        match pos {
            0 => None,
            1 => Some(list.elt),
            _ => {
                while let Some(ref mut next_node) = list.next {
                    if next_node.len == pos {
                        return Some(next_node.elt);
                    }

                    list = next_node;
                }

                None
            }
        }
    }
}

#[cfg(test)]
mod test_linked_list {
    use super::*;

    #[test]
    fn new_list() {
        let list = LinkedList::new(&1);

        assert_eq!(*list.elt, 1);
        assert!(list.next.is_none());
    }

    #[test]
    fn push_elements() {
        let mut list = LinkedList::new(&14);
        list.push(&3);
        list.push(&9);

        assert_eq!(*list.elt, 14);
        assert_eq!(*list.next.as_ref().unwrap().elt, 3);
        assert_eq!(*list.next.as_ref().unwrap().next.as_ref().unwrap().elt, 9);
    }

    #[test]
    fn position_of_elements() {
        let mut list = LinkedList::new(&1);
        list.push(&2);
        list.push(&3);

        assert_eq!(list.len, 1);
        assert_eq!(list.next.as_ref().unwrap().len, 2);
        assert_eq!(list.next.as_ref().unwrap().next.as_ref().unwrap().len, 3);
    }

    #[test]
    fn get_element() {
        let mut list = LinkedList::new(&7);
        list.push(&24);
        list.push(&68);

        assert_eq!(list.get(0), None);
        assert_eq!(list.get(1), Some(&7));
        assert_eq!(list.get(2), Some(&24));
        assert_eq!(list.get(3), Some(&68));
        assert_eq!(list.get(4), None);
    }
}
