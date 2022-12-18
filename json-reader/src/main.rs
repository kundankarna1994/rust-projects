use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Question {
    question: String,
    options: Vec<String>,
    answer: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Questions {
    #[serde(flatten)]
    question: HashMap<String, Question>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Categories {
    #[serde(flatten)]
    questions: HashMap<String, Question>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Quiz {
    #[serde(flatten)]
    quiz: HashMap<String, Categories>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Root {
    #[serde(flatten)]
    quiz: HashMap<String, Quiz>,
}

fn main() {
    let quiz = read_quiz_from_file("example_2.json").unwrap();
    println!("{:?}", quiz);
}

fn read_quiz_from_file<P: AsRef<Path>>(path: P) -> Result<Root, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let q = serde_json::from_reader(reader)?;
    Ok(q)
}
