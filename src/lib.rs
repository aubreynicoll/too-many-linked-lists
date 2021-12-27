pub mod first;

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
}
