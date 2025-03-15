
mod structs;
use structs::array::Arr;


fn arr_test() {

    let mut arr = Arr::malloc(5).unwrap();
    arr.insert(1);
    arr.insert(2);
    arr.insert(3);

    for i in 0..arr.len() {
        if let Some(value) = arr.get(i) {
            println!("element {}: {}", i, value);
        }
    }
}




fn main() {
    arr_test();
}
