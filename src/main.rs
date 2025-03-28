
mod structs;
use structs::array::Arr;
use structs::hash_map::Dict;


fn arr_test() {

    let mut arr = Arr::malloc(5).unwrap();
    arr.insert(1);
    arr.insert(2);
    arr.insert(3);
    arr.remove_mid(); // removes 2

    for i in 0..arr.len() {
        if let Some(value) = arr.get(i) {
            println!("element {}: {}", i, value);
        }
    }
}




fn main() {
    arr_test();

    let mut dict: Dict<&str, i32> = Dict::new(100);

    dict.insert("cool", 32);
    println!("{:?}", dict.get("cool"));
    println!("{}", dict.len());
}
