fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // フィルタリング条件: 偶数だけを抽出
    let filter_even = |x: &&i32| -> bool { *x % 2 == 0 };

    // マッピング処理: 2倍にする
    let map_double = |x: i32| -> i32 { x * 2 };

    let filtered_and_mapped: Vec<i32> = numbers
        .iter()
        .filter(filter_even)
        .map(|&x| map_double(x))
        .collect();

    println!("Filtered and mapped list: {:?}", filtered_and_mapped);
}