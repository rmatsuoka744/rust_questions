fn main() {
    let mut words = vec![
        String::from("apple"),
        String::from("banana"),
        String::from("cherry"),
        String::from("date"),
        String::from("elderberry"),
    ];

    // ソート基準: 文字列の長さの降順

    let sort_by_length = |s: &String, t: &String| t.len().cmp(&s.len());

    words.sort_by(sort_by_length); 

    println!("Sorted words by length (desc): {:?}", words);
}