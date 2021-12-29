pub mod first;
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
}
