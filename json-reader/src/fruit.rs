use std::{fs::File, io::BufReader, path::Path};

use serde::{Deserialize, Serialize};
use std::error::Error;
#[derive(Debug, Serialize, Deserialize)]
struct Fruit {
    name: String,
    size: String,
    color: String,
    quantity: i32,
}

fn main() {
    let u = read_fruit_from_file("example_1.json").unwrap();
    println!("{:?}", u);
}

fn read_fruit_from_file<P: AsRef<Path>>(path: P) -> Result<Fruit, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let u = serde_json::from_reader(reader)?;
    Ok(u)
}
