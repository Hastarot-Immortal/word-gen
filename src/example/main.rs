mod model;
use model::Analyzer;

fn main() {
    let names = TxtReader::read("example/name.txt").unwrap();
    let mut model = model::Model::new();
    model.new_words(names);
    for _ in 0..100 {
        println!("{}", model.random_word_with_range(6..10));
    }
}

pub struct TxtReader;
impl TxtReader {
    fn read(path: &str) -> std::io::Result<Vec<String>> {
        let text = std::fs::read_to_string(path)?;
        let text: Vec<String> = text
            .split(|c| matches!(c, '\n' | '\r'))
            .filter(|s| s != &"" )
            .map(|s| s.to_string().to_lowercase())
            .collect();
        Ok(text)
    }
}