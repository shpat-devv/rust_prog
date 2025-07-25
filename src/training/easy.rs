pub fn array_reverse(arr: &mut [i32]) -> &mut [i32] {
    let mut current_index: usize = 0;
    let mut last_index: usize = arr.len() - 1;

    while current_index < last_index {
        arr.swap(current_index, last_index);
        current_index += 1;
        last_index -= 1;
    }

    arr
}

pub fn find_max(arr: &mut [i32]) -> i32 {
    let mut max_value: i32 = 0;

    for num in arr.iter() {
        if *num > max_value {
            max_value = *num;
        }
    }

    max_value
}

pub fn find_lowest(arr: &mut [i32]) -> i32 {
    let mut lowest_value: i32 = arr[0];

    for num in arr.iter() {
        if *num < lowest_value {
            lowest_value = *num;
        }
    }

    lowest_value
}

pub fn binary_search(arr: &mut [i32], target: i32) -> i32 {
    let mut found: bool = false;
    let mut arr_len: usize = arr.len() - 1;
    let mut i: usize = arr_len / 2;

    while found == false {
        if arr[i] == target {
            found = true;
        } else {
            arr_len = arr_len / 2;
            if arr[i] < target {
                i = i + arr_len;
            } else if arr[i] > target {
                i = i - arr_len;
            }
        }
    }

    i.try_into().unwrap()
}

pub fn bubble_sort(mut list: Vec<i32>) -> Vec<i32> {
    let mut counter = 0;
    let list_size = list.len() - 1;
    while counter < list_size {
        for i in 0..list_size {
            if list[i] > list[i + 1] {
                list.swap(i, i + 1);
            }
        }
        counter += 1;
    }
    list
}

pub fn check_sort(arr: &[i32]) -> bool {
    let len = arr.len() - 1;

    for i in 0..len {
        if arr[i] > arr[i + 1] {
            return false;
        }
    }
    true
}

//find smallest part of unsorted array and then swap it with current index
pub fn selection_sort(mut list: Vec<i32>) -> Vec<i32> {
    let list_size = list.len();
    let mut cache;
    for i in 0..list_size {
        for j in i..list_size {
            if list[j] < list[i] {
                cache = list[i];
                list[i] = list[j];
                list[j] = cache;
            }
        }
    }

    list
}

pub fn recursion(x: i32) {
    if x != 1 {
        recursion(x - 1);
    } else {
        println! {"I have found myself back home"};
    }
}

/*
when given a unsorted list of numbers, return the sorted version

cut the list into pieces


*/
pub fn merge_sort(mut cupcakes: Vec<i32>) -> Vec<i32> {
    if cupcakes.len() < 2 {
        return cupcakes
    } 

    cupcakes
}

fn merge(mut cupcakes: Vec<i32>) -> Vec<i32>{
    if cupcakes[0] > cupcakes[1] {
        let placeholder = cupcakes[1];
        cupcakes[1] = cupcakes[0];
        cupcakes[0] = placeholder;
    }
    cupcakes
}

/*
pub fn selection_sort() {

}
*/
