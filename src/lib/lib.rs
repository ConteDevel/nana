mod tokenizer;

use tokenizer::tokenize;

pub fn run(source: &str) {
    let mut idx: usize = 0;
    while let Some(token) = tokenize(source, &mut idx) {
        println!("{}", token);
    }
}