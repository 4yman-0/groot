use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tokenizer::Tokenizer;
use interpreter::Interpreter;

mod tokenizer;
mod operations;
mod interpreter;

fn main() {
    let mut arguments = env::args();
    arguments.next();
    let filename = arguments.next().expect("Usage: groot file.groot");
    let mut file = File::open(Path::new(&filename)).unwrap_or_else(|_| panic!("Could not open file {}", filename));
    let mut string = String::new();
    file.read_to_string(&mut string).unwrap_or_else(|_| panic!("Could not read file {}", filename));
    let content = string.replace("\n", " ");
    let words = content.split(' ').collect::<Vec<&str>>();
    let mut tokenizer = Tokenizer::new(words);
    let ops = tokenizer.tokenize();
    let mut interpreter = Interpreter::new(ops);
    interpreter.run();
}
