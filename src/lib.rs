pub mod fifth;
pub mod first;
pub mod fourth;
pub mod second;
pub mod third;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_list_works() {
        let mut first_list = first::List::new();

        assert_eq!(None, first_list.pop());

        first_list.push(1);
        first_list.push(2);
        first_list.push(3);

        assert_eq!(Some(3), first_list.pop());
        assert_eq!(Some(2), first_list.pop());

        first_list.push(101);

        assert_eq!(Some(101), first_list.pop());
        assert_eq!(Some(1), first_list.pop());
        assert_eq!(None, first_list.pop());
    }

    #[test]
    fn second_list_works() {
        let mut second_list = second::List::new();

        assert_eq!(None, second_list.pop());

        second_list.push(1);
        second_list.push(2);
        second_list.push(3);

        assert_eq!(Some(3), second_list.pop());
        assert_eq!(Some(2), second_list.pop());

        second_list.push(101);

        assert_eq!(Some(101), second_list.pop());
        assert_eq!(Some(1), second_list.pop());
        assert_eq!(None, second_list.pop());
    }

    #[test]
    fn second_peek_works() {
        let mut second_list = second::List::new();
        assert_eq!(None, second_list.peek());

        second_list.push(1);
        assert_eq!(Some(&1), second_list.peek());

        second_list.push(101);
        assert_eq!(Some(&101), second_list.peek());
    }

    #[test]
    fn second_peek_mut_works() {
        let mut second_list = second::List::new();
        assert_eq!(None, second_list.peek_mut());

        second_list.push(1);
        assert_eq!(Some(&mut 1), second_list.peek_mut());

        second_list.push(101);
        assert_eq!(Some(&mut 101), second_list.peek_mut());

        if let Some(value) = second_list.peek_mut() {
            *value = 7;
        }

        assert_eq!(Some(&mut 7), second_list.peek_mut());
    }

    #[test]
    fn second_into_iter_works() {
        let mut second_list = second::List::new();
        second_list.push(1);
        second_list.push(2);
        second_list.push(3);

        let mut into_iter = second_list.into_iter();

        assert_eq!(Some(3), into_iter.next());
        assert_eq!(Some(2), into_iter.next());
        assert_eq!(Some(1), into_iter.next());
        assert_eq!(None, into_iter.next());
    }

    #[test]
    fn second_iter_works() {
        let mut second_list = second::List::new();
        second_list.push(1);
        second_list.push(2);
        second_list.push(3);

        let mut iter = second_list.iter();

        assert_eq!(Some(&3), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn second_iter_mut_works() {
        let mut second_list = second::List::new();
        second_list.push(1);
        second_list.push(2);
        second_list.push(3);

        let mut iter_mut = second_list.iter_mut();

        while let Some(n) = iter_mut.next() {
            *n = *n * 2;
        }

        let mut into_iter = second_list.into_iter();

        assert_eq!(Some(6), into_iter.next());
        assert_eq!(Some(4), into_iter.next());
        assert_eq!(Some(2), into_iter.next());
        assert_eq!(None, into_iter.next());
    }

    #[test]
    fn third_list_works() {
        let list = third::List::new();

        let list = list.push(1).push(2).push(3);
        assert_eq!(Some(&3), list.head());

        let list = list.tail();
        assert_eq!(Some(&2), list.head());

        let list = list.tail();
        assert_eq!(Some(&1), list.head());

        let list = list.tail();
        assert_eq!(None, list.head());
    }

    #[test]
    fn third_list_iter_works() {
        let list = third::List::new().push(1).push(2).push(3);

        let mut iter = list.iter();

        assert_eq!(Some(&3), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn fourth_list_works() {
        let mut list = fourth::List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_head(), None);

        // Populate list
        list.prepend(1);
        list.prepend(2);
        list.prepend(3);

        // Check normal removal
        assert_eq!(list.pop_head(), Some(3));
        assert_eq!(list.pop_head(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.prepend(4);
        list.prepend(5);

        // Check normal removal
        assert_eq!(list.pop_head(), Some(5));
        assert_eq!(list.pop_head(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_head(), Some(1));
        assert_eq!(list.pop_head(), None);

        let mut list = fourth::List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_tail(), None);

        // Populate list
        list.append(1);
        list.append(2);
        list.append(3);

        // Check normal removal
        assert_eq!(list.pop_tail(), Some(3));
        assert_eq!(list.pop_tail(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.append(4);
        list.append(5);

        // Check normal removal
        assert_eq!(list.pop_tail(), Some(5));
        assert_eq!(list.pop_tail(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_tail(), Some(1));
        assert_eq!(list.pop_tail(), None);
    }

    #[test]
    fn fourth_list_peek_head() {
        let mut list = fourth::List::new();
        assert!(list.peek_head().is_none());
        list.prepend(1);
        list.prepend(2);
        list.prepend(3);

        assert_eq!(&*list.peek_head().unwrap(), &3);
    }

    #[test]
    fn fourth_list_into_iter() {
        let mut list = fourth::List::new();
        list.prepend(1);
        list.prepend(2);
        list.prepend(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn fifth_list_ok() {
        let mut list = fifth::List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn fifth_into_iter_works() {
        let mut fifth_list = fifth::List::new();
        fifth_list.push(1);
        fifth_list.push(2);
        fifth_list.push(3);

        let mut into_iter = fifth_list.into_iter();

        assert_eq!(Some(1), into_iter.next());
        assert_eq!(Some(2), into_iter.next());
        assert_eq!(Some(3), into_iter.next());
        assert_eq!(None, into_iter.next());
    }

    #[test]
    fn fifth_iter_works() {
        let mut fifth_list = fifth::List::new();
        fifth_list.push(1);
        fifth_list.push(2);
        fifth_list.push(3);

        let mut iter = fifth_list.iter();

        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&3), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn fifth_iter_mut_works() {
        let mut fifth_list = fifth::List::new();
        fifth_list.push(1);
        fifth_list.push(2);
        fifth_list.push(3);

        let mut iter_mut = fifth_list.iter_mut();

        while let Some(n) = iter_mut.next() {
            *n = *n * 2;
        }

        let mut into_iter = fifth_list.into_iter();

        assert_eq!(Some(2), into_iter.next());
        assert_eq!(Some(4), into_iter.next());
        assert_eq!(Some(6), into_iter.next());
        assert_eq!(None, into_iter.next());
    }
}
