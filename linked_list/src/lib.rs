#[cfg(test)]
mod tests {
    
    struct List {
        empty : bool,
        item : i64,
    }


    impl List {
        fn new() -> List {
            List { empty : true, item : 0 }
            }
        fn empty(&self) -> bool {
            self.empty
        }
        fn cons(&mut self, item : i64) {
            if !self.empty {
                
            }
            self.item = item;
            self.empty = false;
        }
        fn head(&self) -> Option <i64> {
            if self.empty() {
                None
            }
            else {
                Some(self.item)
            }
        }
        fn tail(&self) -> List {
            List::new()
        }

    }
    #[test]
    fn is_initially_empty() {
        let list = List::new();
        assert_eq!(true, list.empty())
    }
    #[test]
    fn cons_an_item_makes_the_list_not_empty() {
        let mut list = List::new();
        list.cons(42);
        assert_eq!(false, list.empty())
    }
    #[test]
    fn head_on_empty_list_yields_nothing() {
        let list = List::new();
        assert_eq!(None, list.head());
    }
    #[test]
    fn head_on_non_empty_list_yields_some_item() {
        let mut list = List::new();
        list.cons(42);
        assert_eq!(Some(42), list.head());
    }
    #[test]
    fn head_on_non_empty_list_yields_any_consed_item() {
        let mut list = List::new();
        list.cons(4807);
        assert_eq!(Some(4807), list.head());
        // check that we can use head several times
        assert_eq!(Some(4807), list.head());
    }
    #[test]
    fn head_after_several_consed_yields_last_consed_item() {
        let mut list = List::new();
        list.cons(4807);
        list.cons(42);
        assert_eq!(Some(42), list.head());
    }
    #[test]
    fn tail_of_an_empty_list_is_an_empty_list() {
        let list = List::new();
        assert_eq!(true, list.tail().empty());
    }
    #[test]
    fn tail_of_a_singleton_list_is_an_empty_list() {
        let mut list = List::new();
        list.cons(42);
        assert_eq!(true, list.tail().empty());
    }
    #[test]
    fn tail_after_two_cons_yields_list_with_first_consed_item() {
        let mut list = List::new();
        list.cons(4807);
        list.cons(42);
        assert_eq!(Some(4807), list.tail().head());
    }

}
