use std::{collections::HashMap, fs, io::Error};

fn main() {
    let mut arguements = std::env::args().skip(1);
    let key = arguements.next().expect("Key was not there");
    let value = arguements.next().unwrap();
    println!("The key is '{}' and the value is '{}'", key, value);
    let contents = format!("{}\t{}\n", key, value);
    fs::write("store.txt", contents).unwrap();
    let database = Database::new().expect("Database new crashed");
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, Error> {
        let mut map = HashMap::new();
        let contents = fs::read_to_string("store.txt")?;
        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("corrupt database");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database { map: map })
    }
}
