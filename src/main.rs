
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

fn dict_test() {

    let mut dict: Dict<&str, i32> = Dict::new(100);
    dict.insert("0001", 4);
    dict.insert("0010", 5);
    dict.insert("0100", 6);
    let keys = ["0001", "0010", "0100"];

    for i in 0..dict.len() {
        let key = keys[i];
        if let Some(value) = dict.get(key) {
            println!("key: {}, value: {}", key, value);
        }
    }

    dict.remove("0001");

    println!("{:?}", dict.get("0100"));
    println!("{:?}", dict.get("0001"));
    println!("{}", dict.len());
}



fn main() {
    arr_test();
    println!("");
    dict_test();

}
