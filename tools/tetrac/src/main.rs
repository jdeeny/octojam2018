use std::io::Write;
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

    let mut source = File::open("build/tetra_input.t4").unwrap();
    let mut header_dest = File::create("build/tetra_header.o8").unwrap();
    let mut data_dest = File::create("build/tetra_data.o8").unwrap();

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


    dump_tetra_header(&mut header_dest);
    dump_dict_consts(&mut header_dest, &dictionary);

    // Output the dictionary definitions
    dump_dict(&mut data_dest, &dictionary);

    // Dump the dictionary table
    dump_dict_table(&mut data_dest, &dictionary);

}


fn define(dictionary: &mut HashMap<Word, Definition>, word: &Word, definition: &Definition) {
    assert!(dictionary.get(word) == None);
    dictionary.insert(word.clone(), definition.clone());
}

fn dump_dict_consts(out: &mut Write, dictionary: &HashMap<Word, Definition>) {
    writeln!(out, "\n\n: TETRA_DICT_CONSTANTS  # Indexes into the table");
    for (i, (key, value)) in dictionary.iter().enumerate() {
        let Word(name) = key;
        let name = name.to_uppercase();
        let s = format!(":calc {}  {{ {} }}", name, i);
        writeln!(out, "{:30}# {} {:?} - {:?}", s, i, key, value);
    }

    writeln!(out, "\n\n# Instantiations of native tetra words");
    for (Word(key), value) in dictionary {
        if let Definition::OctoCall(target) = value {
            match target {
                WordOrLiteral::W(Word(w)) => {
                    writeln!(out, "impl_{}", w);
                },
                WordOrLiteral::L(l) => {},
            }
        }
    }

}

fn dump_dict_table(out: &mut Write, dictionary: &HashMap<Word, Definition>) {
    writeln!(out, "\n\n: tetra_dictionary   # Begin Dictionary Table  Entries: 1B Type, 2B Data");
    for (i, (key, value)) in dictionary.iter().enumerate() {
        let Word(name) = key;
        let name = name.to_uppercase();
        let s = format!(":calc {}  {{ {} }}", name, i);
        writeln!(out, "{:30}# {} {:?} - {:?}", s, i, key, value);
        dump_tetra_table(out, &name, &value);
    }
}

fn dump_dict(out: &mut Write, dictionary: &HashMap<Word, Definition>) {
    writeln!(out, "\n\n# Begin Tetra Dictionary Entries");
    for (key, value) in dictionary {
        let Word(name) = key;
        let name = name.to_uppercase();
//        let s = format!(":calc {} {{ {} }}", name, i);
//        println!("{:30} # {} {:?} - {:?}", s, i, key, value);
        dump_tetra_definition(out, &name, &value);
    }
}


fn dump_tetra_table(out: &mut Write, name: &str, def: &Definition) {
    let defname = String::from("T4_") + name;
    match def {

        Definition::Tetra(_)    => { writeln!(out, "DEF_TETRA   T4_DUMP_U16   {}", defname); },
        Definition::OctoCall(_) => { writeln!(out, "DEF_CALL    T4_DUMP_U16   {}", name); },
        Definition::OctoAddr(_) => { writeln!(out, "DEF_ADDR    T4_DUMP_U16   {}", name); },
        Definition::Literal(n)  => { writeln!(out, "DEF_LITERAL {} {}", n >> 8, n & 0xFF ); },

    }

}



fn dump_tetra_definition(out: &mut Write, name: &str, def: &Definition) {
    let defname = String::from("T4_") + name;
    match def {
        Definition::Tetra(d)    => {
            write!(out, ": {}", defname);
            for w in d {
                match w {
                    WordOrLiteral::W(Word(word)) => { write!(out, " {}", word); }
                    WordOrLiteral::L(l)      => { write!(out, " T4_LIT_{}", l); },
                }
            }
            writeln!(out, "");
        },
        _ => {},
    }

}

fn dump_tetra_header(out: &mut Write) {
    // Constants for the table
    writeln!(out, ":calc DEF_TETRA   {{ 0 }}");
    writeln!(out, ":calc DEF_CALL    {{ 1 }}");
    writeln!(out, ":calc DEF_ADDR    {{ 2 }}");
    writeln!(out, ":calc DEF_LITERAL {{ 3 }}");

    // Macro that dumps the value of an identifier into the source as two bytes
    writeln!(out, ":macro T4_DUMP_U16 val {{ :calc hi {{ val >> 8 }} :calc lo {{ val & 0xFF }} :byte hi :byte lo }}");

}
