pub mod list {
    pub struct List {
        head: *mut Item,
        size: u32,
    }

    impl List {
        pub fn of(head: i32) -> List {
            let size = 1;
            List {
                head: &mut Item::new(head),
                size: size,
            }
        }

        pub fn size_of(self: &List) -> u32 {
            self.size
        }

        /// For now, return value maintained by head.
        /// TODO(ebwb): once #add is implemented, will need to traverse list.
        pub fn get_at(self: &List, index: u32) -> i32 {
            self.head.value
        }

        pub fn add(self: &mut List, to_add: u32) {
            self.size += 1;
        }
    }

    struct Item {
        value: i32,
        next: *mut Option<Item>,
        prev: *mut Option<Item>,
    }

    impl Item {
        fn new(value: i32) -> Item {
            Item {
                value: value,
                next: &mut None,
                prev: &mut None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::list::List;

    #[test]
    fn size_of_fresh_construction() {
        let list = List::of(1);

        assert_eq!(1, list.size_of());
    }

    mod get_at_tests {
        use super::*;

        #[test]
        fn get_at_in_bounds_returns_value() {
            todo!("write the test!");
        }

        #[test]
        fn get_at_left_edge() {
            let mut list = List::of(5);
            list.add(4);
            list.add(3);

            assert_eq!(5, list.get_at(0));
        }

        #[test]
        fn get_at_right_bound() {
            let mut list = List::of(3);
            list.add(2);
            list.add(1);

            assert_eq!(1, list.get_at(2));
        }

        #[test]
        fn get_at_out_of_bounds() {
            todo!("write the test!");
        }
    }
}
