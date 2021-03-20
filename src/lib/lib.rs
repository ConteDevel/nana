mod fsm;
mod tokenizer;

use tokenizer::Tokenizer;

pub fn run(source: &str) {
    let mut idx: usize = 0;
    let mut tokenizer = Tokenizer::new();
    while let Some(token) = tokenizer.next(&source, &mut idx) {
        println!("{}", token);
    }
}