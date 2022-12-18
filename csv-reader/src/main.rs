use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::process;

#[allow(unused)]
#[derive(Debug, Deserialize)]
struct Record {
    id: Option<String>,
    title: Option<String>,
    author: Option<String>,
    downloads: Option<String>,
    lat: Option<String>,
    lng: Option<String>,
    accuracy: Option<String>,
    location: Option<String>,
    category: Option<String>,
    density: Option<String>,
}
fn main() {
    if let Err(e) = read_from_file("./sample.csv") {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn read_from_file(file: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&file);
    println!("{:?}", path);
    let file = File::open(path)?;

    let mut reader = csv::Reader::from_reader(file);

    for result in reader.records() {
        let record = result?;
        let id = record.get(0).map(str::to_string);
        let title = record.get(1).map(str::to_string);
        let author = record.get(2).map(str::to_string);
        let downloads = record.get(3).map(str::to_string);
        let lat = record.get(4).map(str::to_string);
        let lng = record.get(5).map(str::to_string);
        let accuracy = record.get(6).map(str::to_string);
        let location = record.get(7).map(str::to_string);
        let category = record.get(8).map(str::to_string);
        let density = record.get(9).map(str::to_string);

        let result = Record {
            id,
            title,
            author,
            downloads,
            lat,
            lng,
            accuracy,
            location,
            category,
            density,
        };
        println!("{:?}", result);

        // println!("{:?}", record);
    }
    Ok(())
}
