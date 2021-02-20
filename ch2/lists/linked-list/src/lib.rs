pub mod list {
    pub struct List<'a> {
        head: Item<'a>,
        size: u32,
    }
    
    impl <'a> List<'a> {
        pub fn of(head: i32) -> List<'a> {
            let size = 1;
            List{
                head: Item::new(head),
                size: size,
            }
        }

        pub fn size_of(self: &'a List<'a>) -> u32 {
            self.size
        }
    }

    struct Item<'a> {
        value: i32,
        next: &'a Option<Item<'a>>,
    }

    impl <'a> Item<'a> {
        fn new(value: i32) -> Item<'a> {
            Item {
                value: value,
                next: &None,
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
}


