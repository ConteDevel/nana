mod fsm;
mod tokenizer;

use tokenizer::Tokenizer;
use indexmap::IndexMap;

pub fn run(source: &str) {
    let mut idx: usize = 0;
    let mut vars = IndexMap::<String, u32>::new();
    let mut tokenizer = Tokenizer::new(&mut vars);
    while let Some(token) = tokenizer.next(&source, &mut idx) {
        println!("{:?}", token);
    }
    println!("{:?}", vars);
}