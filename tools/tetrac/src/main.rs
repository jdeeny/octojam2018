
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

use derive_more::{ From, Into };
mod parse;

#[derive(From, Into, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Word(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Definition {
    /// A pointer to a different word (for aliasing later)
    Pointer(Word),
    /// A series of words
    Words(Vec<Word>),
}

pub struct DictEntry {
    key: Word,
    value: Definition,
}

fn main() {
    println!("Hello, world!");

    let mut source = File::open("testfile.t4").unwrap();

    let mut dictionary: HashMap<Word, Definition> = HashMap::new();
    let mut reverse_dictionary: HashMap<Definition, Word>  = HashMap::new();

    let mut buffer = String::new();
    let result = source.read_to_string(&mut buffer);

    let parsed = parse::tetra_source(&buffer);

    println!("{:?}", parsed);

}


fn define(dictionary: &mut HashMap<Word, Definition>, reverse_dictionary: &mut HashMap<Definition, Word>, word: &Word, definition: &Definition) {
    assert!(dictionary.get(word) == None);
    if let Some(existing_word) = reverse_dictionary.get(definition) {
        dictionary.insert(word.clone(), Definition::Pointer(existing_word.clone()));
        return;
    }
    dictionary.insert(word.clone(), definition.clone());
}
