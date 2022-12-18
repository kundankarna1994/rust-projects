use std::{fs::File, io::Write};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Paragraph {
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Article {
    title: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() -> std::io::Result<()> {
    let article: Article = Article {
        title: "How to work with json".to_string(),
        author: "Kundan Karna".to_string(),
        paragraph: vec![
            Paragraph {
                name: "Header Paragraph".to_string(),
            },
            Paragraph {
                name: "Body Paragraph".to_string(),
            },
            Paragraph {
                name: "Footer Paragraph".to_string(),
            },
        ],
    };

    let json = serde_json::to_string(&article).unwrap();
    let mut file = File::create("example.json")?;
    file.write_all(json.as_bytes())?;
    println!("The json is {}", json);
    Ok(())
}
