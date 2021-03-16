pub mod list {
    use std::cell::RefCell;
    use std::borrow::{BorrowMut, Borrow};
    use std::rc::Rc;

    pub struct List {
        head: Option<Rc<RefCell<Item>>>,
        tail: Option<Rc<RefCell<Item>>>,
        size: u32,
    }

    impl List {
        pub fn of(head: i32) -> List {
            let size = 1;
            List {
                head: Some(Rc::new(RefCell::new(Item::of(head)))),
                tail: None,
                size,
            }
        }

        pub fn size_of(self: &List) -> u32 {
            self.size
        }

        /// For now, return value maintained by head.
        /// TODO(ebwb): once #add is implemented, will need to traverse list.
        pub fn get_at(self: &List, index: u32) -> i32 {
            match &self.head {
                Some(item) => RefCell::borrow(item.clone().borrow()).value,
                None => 0,
            }
            /*match self.head.borrow() {
                Some(item) => item.clone().borrow(),
                None => 0,  //TODO(ebwb): certainly not permanent!
            }*/
        }

        // TODO(ebwb): implement not just size, but actual addition
        // TODO(ebwb): interior mutability should let us not need a mutable reference
        pub fn push_front(self: &mut List, to_add: i32) {
            // starting from head
            // go until we find tail (next returns none, not some)
            // at that point
            //    create new item
            //    assign item to old tail's 'next'
            //    set list's tail to new item
            //TODO(ebwb): size == 0 option?

            let mut new_head = Item::of(to_add);

            match self.head.take() {
                Some(old_head) => {
                    new_head.borrow_mut().next = Some(Rc::clone(&old_head));
                }
                None => { println!("add: none"); }
            }

            self.size = self.size + 1;
        }

        pub fn push_back(self: &mut List, to_add: i32) {
            let mut new_tail = Item::of(to_add);

            match self.tail.take() {
                Some(old_tail) => {
                    new_tail.borrow_mut().next = Some(Rc::clone(&old_tail));

                }
                None => { println!("add: none"); }
            }

            self.size = self.size + 1;
        }
    }

    #[derive(Debug)]
    struct Item {
        value: i32,
        next: Option<Rc<RefCell<Item>>>,
    }

    impl Item {
        fn of(value: i32) -> Item {
            Item {
                value,
                next: None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::list::List;

    mod size_of_tests {
        use super::*;

        #[test]
        fn size_of_fresh_construction() {
            let list = List::of(1);

            assert_eq!(1, list.size_of());
        }
    }

    mod get_at_tests {
        use super::*;

        #[test]
        fn in_bounds_returns_value() {
            let mut list = List::of(5);
            list.push_front(4);
            list.push_front(3);

            assert_eq!(4, list.get_at(1));
        }

        #[test]
        fn left_edge() {
            let mut list = List::of(5);
            list.push_front(4);
            list.push_front(3);

            assert_eq!(5, list.get_at(0));
        }

        #[test]
        fn right_bound() {
            let mut list = List::of(3);
            list.push_front(2);
            list.push_front(1);

            assert_eq!(1, list.get_at(2));
        }

        #[test]
        fn out_of_bounds() {
            todo!("write the test!");
        }
    }
}
