mod tokenizer;

use tokenizer::Tokenizer;
use indexmap::IndexMap;

pub fn run(source: &str) {
    let mut idx: usize = 0;
    let vars = IndexMap::<String, u32>::new();
    let tokenizer = Tokenizer::new();
    while let Some(token) = tokenizer.next(&source, &mut idx) {
        println!("{:?}", token);
    }
    println!("{:?}", vars);
}