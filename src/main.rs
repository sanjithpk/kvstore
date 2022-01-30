use std::fs;

fn main() {
    let mut arguements = std::env::args().skip(1);
    let key = arguements.next().expect("Key was not there");
    let value = arguements.next().unwrap();
    println!("The key is '{}' and the value is '{}'", key, value);
    let contents = format!("{}\t{}\n", key, value);
    fs::write("store.txt", contents).unwrap();
}
