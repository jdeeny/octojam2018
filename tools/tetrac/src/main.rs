use std::ops::Add;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

use derive_more::{ From, Into };
mod parse;


#[derive(From, Into, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Word(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WordOrLiteral{
    W(Word),
    L(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Definition{
    Tetra(Vec<WordOrLiteral>),
    OctoCall(WordOrLiteral),
    OctoAddr(WordOrLiteral),
    Literal(usize),
}

#[derive(From, Into, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DictEntry {
    pub key: Word,
    pub value: Definition,
}



fn main() {

    let mut source = File::open("testfile.t4").unwrap();

    let mut dictionary: HashMap<Word, Definition> = HashMap::new();

    let mut buffer = String::new();
    let _ = source.read_to_string(&mut buffer);

    buffer = buffer + ":bandaid shit #  \n\n";

    println!("{:?} \n  -->", buffer);


    let (_, parsed) = parse::tetra_source(&buffer).unwrap();
    for entry in parsed {
        let key = entry.key;
        let value = entry.value;

        define(&mut dictionary, &key, &value);
    }

    println!("{:?}", dictionary);

    dump_dict_octo(&dictionary);

}


fn define(dictionary: &mut HashMap<Word, Definition>, word: &Word, definition: &Definition) {
    assert!(dictionary.get(word) == None);
    dictionary.insert(word.clone(), definition.clone());
}


fn dump_dict_octo(dictionary: &HashMap<Word, Definition>) {
    println!(":calc TETRA_DICT_BEGINS {{ HERE }}");
    for (i, (key, value)) in dictionary.iter().enumerate() {
        let Word(name) = key;
        let name = name.to_uppercase();
        println!(":calc {} {{ {} }}", name, i);
        println!("   # {} {:?} - {:?}", i, key, value);
        dump_tetra_table(&name, &value);
    }
}


fn dump_tetra_table(name: &str, def: &Definition) {
    let defname = String::from("T4_") + name;
    match def {

        Definition::Tetra(_)    => { println!("DEF_TETRA hilo dump_u16 ( {} )", defname); },
        Definition::OctoCall(_) => { println!("DEF_CALL hilo dump_u16 ( {} )", name); },
        Definition::OctoAddr(_) => { println!("DEF_ADDR hilo dump_u16 ( {} )", name); },
        Definition::Literal(n)  => { println!("DEF_LITERAL {} {}", n >> 8, n & 0xFF ); },

    }

}


/*
fn dump_octo_definition(name: &str, def: &Definition) {
    let defname = String("T4_") + name;
    match def {

        Definition::Tetra(_) => {    println!("DEF_TETRA hilo dump_u16 ( {} ) ", defname);
},
        Definition::OctoCall(_) => {    println!("DEF_CALL ADDRHI ADDRLO");
},
        Definition::OctoAddr(_) => {    println!("DEF_ADDR ADDRHI ADDRLO");
},
        Definition::Literal(n) => {    println!("DEF_LITERAL HI LO");
},

    }

}
*/
