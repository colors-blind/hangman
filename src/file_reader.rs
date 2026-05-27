use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::Rng;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_list(filename: &str) -> Vec<String> {
    let mut v = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for w in lines.flatten() {
            if w.len() > 4 {
                v.push(w);
            }
        }
    }
    v
}

pub fn select_word() -> String {
    let mut rng = rand::thread_rng();
    let words = read_list("words.txt");
    if words.is_empty() {
        return "hangman".to_string();
    }
    let selection = rng.gen_range(0..words.len());
    words[selection].clone()
}
