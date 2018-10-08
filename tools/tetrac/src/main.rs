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

    let (_, parsed) = parse::tetra_source(&buffer).unwrap();
    for entry in parsed {
        let key = entry.key;
        let value = entry.value;
        define(&mut dictionary, &key, &value);
    }



    //search for literals
    let mut literals = Vec::<usize>::new();
    for def in dictionary.values() {
        if let Definition::Tetra(def) = def {
            for word in def.iter() {
                if let WordOrLiteral::L(l) = word {
                    literals.push(*l);
                }
            }
        }
    }

    for l in literals {
        let name = format!("T4_LIT_{}", l);
        if dictionary.get(&Word(name.clone()))  == None {
            define(&mut dictionary, &Word(name), &Definition::Literal(l));
        }
    }


    dump_tetra_header();

    // Output the dictionary definitions
    dump_dict(&dictionary);

    // Dump the dictionary table
    dump_dict_table(&dictionary);

}


fn define(dictionary: &mut HashMap<Word, Definition>, word: &Word, definition: &Definition) {
    assert!(dictionary.get(word) == None);
    dictionary.insert(word.clone(), definition.clone());
}


fn dump_dict_table(dictionary: &HashMap<Word, Definition>) {
    println!("\n\n: TETRA_DICT   # Begin Dictionary Table  Entries: 1B Type, 2B Data");
    for (i, (key, value)) in dictionary.iter().enumerate() {
        let Word(name) = key;
        let name = name.to_uppercase();
        let s = format!(":calc {}  {{ {} }}", name, i);
        println!("{:30}# {} {:?} - {:?}", s, i, key, value);
        dump_tetra_table(&name, &value);
    }
}

fn dump_dict(dictionary: &HashMap<Word, Definition>) {
    println!("\n\n# Begin Tetra Dictionary Entries");
    for (key, value) in dictionary {
        let Word(name) = key;
        let name = name.to_uppercase();
//        let s = format!(":calc {} {{ {} }}", name, i);
//        println!("{:30} # {} {:?} - {:?}", s, i, key, value);
        dump_tetra_definition(&name, &value);
    }
}


fn dump_tetra_table(name: &str, def: &Definition) {
    let defname = String::from("T4_") + name;
    match def {

        Definition::Tetra(_)    => { println!("DEF_TETRA   T4_DUMP_U16   {}", defname); },
        Definition::OctoCall(_) => { println!("DEF_CALL    T4_DUMP_U16   {}", name); },
        Definition::OctoAddr(_) => { println!("DEF_ADDR    T4_DUMP_U16   {}", name); },
        Definition::Literal(n)  => { println!("DEF_LITERAL {} {}", n >> 8, n & 0xFF ); },

    }

}



fn dump_tetra_definition(name: &str, def: &Definition) {
    let defname = String::from("T4_") + name;
    match def {
        Definition::Tetra(d)    => {
            print!(": {}", defname);
            for w in d {
                match w {
                    WordOrLiteral::W(Word(word)) => { print!(" {}", word); }
                    WordOrLiteral::L(l)      => { print!(" T4_LIT_{}", l); },
                }
            }
            println!("");
        },
        _ => {},
    }

}

fn dump_tetra_header() {
    // Constants for the table
    println!(":calc DEF_TETRA   {{ 0 }}");
    println!(":calc DEF_CALL    {{ 1 }}");
    println!(":calc DEF_ADDR    {{ 2 }}");
    println!(":calc DEF_LITERAL {{ 3 }}");

    // Macro that dumps the value of an identifier into the source as two bytes
    println!(":macro T4_DUMP_U16 val {{ :calc hi {{ val >> 8 }} :calc lo {{ val & 0xFF }} :byte hi :byte lo }}");

}
