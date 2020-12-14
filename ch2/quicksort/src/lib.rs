use rand::prelude::*;

pub fn sort(list: &mut [i32], end: usize) {

    println!("list: {:?}", list);
    println!("end: {:?}", end);
    
    if end <= 1 {
	println!("found end, returning");
        return;
    }
    
    let mut rng = thread_rng();
    let random_index : usize = rng.gen_range(0, end);
    let _random_index = end;
    println!("pivot: {}", random_index);
    
    swap(list, 0, random_index);
    println!("first swap: {:?}", list);
    let mut last = 0;

    for i in 1..end {
	if list[i] < list[0] {
	    last += 1;
	    swap(list, last, i);
	}
    }

    
    println!("last: {:?}", last);
    swap(list, 0, last);
    println!("now list: {:?}", list);
    println!("sorting lower half");
    sort(&mut list[0..last], last);
    println!("sorting upper half");
    sort(&mut list[last..end], end-last);
}

// assumes both i and j are valid elements in list. panics if not.
fn swap(list: &mut [i32], i: usize, j: usize) {
    let temp = list[i];
    list[i] = list[j];
    list[j] = temp;
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emptyList_isSorted() {
        let mut empty_list: [i32; 0] = [];
	let length = empty_list.len();

	sort(&mut empty_list, length);

        assert_eq!(Vec::<i32>::new(), empty_list);
    }

    #[test]
    fn listOfOne_isSorted() {
        let mut list_of_one: [i32; 1] = [5];
	let length = list_of_one.len();

	sort(&mut list_of_one, length);
        assert_eq!([5], list_of_one);
    }

    #[test]
    fn smallList_sortsAscending() {
        let mut small_list: [i32; 3] = [6, 0, 1];
        let expected_list: [i32; 3] = [0, 1, 6];
	let length = small_list.len();

	sort(&mut small_list, length);
        assert_eq!(expected_list, small_list);
    }

    #[test]
    fn sortedList_remainsSorted() {
        let mut sorted_list: [i32; 5] = [1, 13, 193, 943, 93139];
        let expected_list: [i32; 5] = [1, 13, 193, 943, 93139];
	let length = sorted_list.len();

	sort(&mut sorted_list, length);
        assert_eq!(expected_list, sorted_list);
    }

    #[test]
    fn reverseList_sortsAscending() {
        let mut reverse_list: [i32; 6] = [999, 943, 931, 64, 32, 1];
        let expected_list: [i32; 6] = [1, 32, 64, 931, 943, 999];
	let length = reverse_list.len();

	sort(&mut reverse_list, length);
	
        assert_eq!(expected_list, reverse_list);
    }

    #[test]
    fn smallList_swap() {
	let mut small_list : [i32; 2] = [0, 1];
	let expected_list : [i32; 2] = [1, 0];
	swap(&mut small_list, 0, 1);

	assert_eq!(expected_list, small_list);
    }

    #[test]
    fn largerList_swap() {
	let mut larger_list : [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
	let expected_list : [i32; 8] = [1, 2, 7, 4, 5, 6, 3, 8];

	swap(&mut larger_list, 2, 6);

	assert_eq!(expected_list, larger_list);
    }

    #[test]
    fn reallyLargeList_sorts() {
	let mut larger_list: Vec<i32> = vec![];
	let mut rng = thread_rng();
	
	
	for _i in 0..1000 {
	    larger_list.push(rng.gen_range(0, 1000000));
	}


	sort(&mut larger_list, 1000);
	
	for i in 0..999 {
	    assert!(larger_list[i] < larger_list[i+1]);
	}
    }
}

