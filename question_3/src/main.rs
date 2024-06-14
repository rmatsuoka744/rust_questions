fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let iter = numbers.iter();
    println!("{:?}", iter);

    let filter_even = |x: &&i32| -> bool { *x % 2 == 0 };

    let filtered_iter = iter.filter(filter_even);

    println!("{:?}", filtered_iter);

    let map_iter = |x: &i32| -> i32 { x * 2 };

    let filtered_maped_iter = filtered_iter.map(map_iter);

    println!("{:?}", filtered_maped_iter);
    
    let filtered_maped_numbers: Vec<i32> = filtered_maped_iter.collect();

    println!("{:?}", filtered_maped_numbers);

}
