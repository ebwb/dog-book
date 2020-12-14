pub fn sort(mut list: &Vec<i32>) -> &Vec<i32> {
    list
}


#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emptyList_isSorted() {
        let empty_list: Vec<i32> = vec![];

        assert_eq!(Vec::<i32>::new(), *sort(&empty_list));
    }

    #[test]
    fn listOfOne_isSorted() {
        let list_of_one: Vec<i32> = vec![5];

        assert_eq!(list_of_one, *sort(&list_of_one));
    }

    #[test]
    fn smallList_sortsAscending() {
	let small_list : Vec<i32> = vec![6, 0, 1];
	let expected_list : Vec<i32> = vec![0, 1, 6];
	
	assert_eq!(expected_list, *sort(&small_list));
    }

    #[test]
    fn sortedList_remainsSorted() {
	let sorted_list : Vec<i32> = vec![1, 13, 193, 943, 93139];
	let expected_list : Vec<i32> = vec![1, 13, 193, 943, 93139];

	assert_eq!(expected_list, *sort(&sorted_list));
    }

    #[test]
    fn reverseList_sortsAscending() {
	let reverse_list : Vec<i32> = vec![999, 943, 931, 64, 32, 1];
	let expected_list : Vec<i32> = vec![1, 32, 64, 931, 943, 999];
	
	assert_eq!(expected_list, *sort(&reverse_list));
    }
}
