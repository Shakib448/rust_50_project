use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]

struct Paragraph {
    name: String,
}

#[derive(Deserialize, Serialize)]

struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let article: Article = Article {
        article: String::from("Muktadir is javascript developer and love to write code with rust"),
        author: String::from("Muktadir Ahamde Shakib"),
        paragraph: vec![
            Paragraph {
                name: String::from("First paragraph"),
            },
            Paragraph {
                name: String::from("Second paragraph"),
            },
            Paragraph {
                name: String::from("Third paragraph"),
            },
        ],
    };

    let json: String = serde_json::to_string(&article).unwrap();
    println!("The json is {}", json)
}
