use std::io;

fn main() {
    let mut arr = String::new();
    let mut vec1: Vec<i64> = Vec::new();
    io::stdin()
        .read_line(&mut arr)
        .expect("Error");
    let mut arr: Vec<&str> = arr.trim().split_whitespace().collect();
    for i in &arr {
        vec1.push( i.parse().unwrap() );
    }
    println!("{:?}", vec1);
}