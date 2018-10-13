use std::io::Write;
use std::ops::Add;
use std::io::prelude::*;
use std::fs::File;
use std::collections::{HashMap, BTreeMap};

use derive_more::{ From, Into };
mod parse;


#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Word {
    pub name: String,
    pub def: Vec<DefPiece>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum DefPiece {
    Tetra(String),
    Literal(usize),
    Token(usize),
    OctoCall(String),
    OctoAddr(String),
}

pub struct Escape {
    value: usize,
    name: String,
}

fn main() {

    let escapes = vec!( Escape{ value: 0xFF, name: String::from("TETRA_ESCAPE_LIT8") },
                        Escape{ value: 0xFE, name: String::from("TETRA_ESCAPE_TETRAPTR") },
                        Escape{ value: 0xFD, name: String::from("TETRA_ESCAPE_OCTOCALL") },
                        Escape{ value: 0xFC, name: String::from("TETRA_ESCAPE_OCTODATA") },
    );


    let mut source = File::open("build/tetra_input.t4").unwrap();
    let mut header_dest = File::create("build/tetra_header.o8").unwrap();
    let mut data_dest = File::create("build/tetra_data.o8").unwrap();

    let mut uses: HashMap<String, usize> = HashMap::new();

    let mut dictionary: HashMap<String, Word> = HashMap::new();

    let mut tokens: BTreeMap<String, usize> = BTreeMap::new();

    let mut buffer = String::new();
    let _ = source.read_to_string(&mut buffer);

    buffer = buffer + ":bandaid shit #  \n\n";



    let (_, parsed) = parse::tetra_source(&buffer).unwrap();


    // count the uses
    for entry in &parsed {
        for w in entry.def.clone() {
            if let DefPiece::Tetra(def) = w {
                *uses.entry(String::from(def)).or_insert(0) += 1;
            }

        }
    }

    let mut sorted_uses: Vec<(String, usize)> =  Vec::new();
    for (uses, word) in uses {
        sorted_uses.push((uses,word));
    }
    sorted_uses.sort_by_key(|v| v.1);

    let mut token_count = 0;

    for (word, count) in sorted_uses {
        tokens.insert(word.clone(), token_count);
        token_count += 1;
        if token_count == 0xFD { break; }
    }


    for w in &parsed {
        dictionary.insert(w.name.clone(), w.clone());
        //define(&mut dictionary, &mut tokens, &k, &v);
    }


    dump_tetra_header(&mut header_dest, &escapes);
    dump_token_consts(&mut header_dest, &tokens);

    dump_dict_entries(&mut data_dest, &dictionary);
    dump_token_table(&mut data_dest, &dictionary, &tokens);
}



fn dump_tetra_header(out: &mut Write, escapes: &Vec<Escape>) {
    for e in escapes {
        writeln!(out, ":calc {} {{ {:16} }}", e.name, e.value);

    }
}

fn dump_token_consts(out: &mut Write, tokens: &BTreeMap<String, usize>) {
    for (name, val) in tokens {
        writeln!(out, ":calc {} {{ {} }}", name, val);
    }
}

/*fn dump_tetra_table(out: &mut Write, name: &str, def: &Definition) {
    let defname = String::from("T4_") + name;
    match def {

        Definition::Tetra(_)    => { writeln!(out, "DEF_TETRA   tobytes   {}", defname); },
        Definition::OctoCall(WordOrLiteral::W(Word(addr))) => { writeln!(out, "DEF_CALL    tobytes   {}", addr); },
        Definition::OctoAddr(WordOrLiteral::W(Word(addr))) => { writeln!(out, "DEF_ADDR    tobytes   {}", addr); },
        Definition::Literal(n)  => { writeln!(out, "DEF_LITERAL {} {}", n >> 8, n & 0xFF ); },
        _ => {},

    }

}*/


fn dump_dict_entries(out: &mut Write, dictionary: &HashMap<String, Word>) {
    writeln!(out, "\n\n# Begin Tetra Dictionary Entries");
    for (name, value) in dictionary {
        let name = name.to_uppercase();
        dump_tetra_definition(out, &name, &value);
    }
}



fn dump_tetra_definition(out: &mut Write, name: &str, word: &Word) {
    let defname = String::from("T4_") + name;

    write!(out, ": {}", defname);
    for piece in word.def.clone() {
        match piece {
            DefPiece::Token(n) => { write!(out, " 0x{:16}", n); },
            DefPiece::Tetra(s) => { write!(out, " TETRA_ESCAPE_TETRAPTR {}", s); },
            DefPiece::Literal(n) => { write!(out, "TETRA_ESCAPE_LIT8 0x{:16}", n); },
            DefPiece::OctoCall(addr) => { write!(out, "TETRA_ESCAPE_OCTOCALL tobytes {}", addr); },
            DefPiece::OctoAddr(addr) => { write!(out, "TETRA_ESCAPE_OCTODATA tobytes {}", addr); },
        }
    }
    write!(out, " EXIT");
    writeln!(out, "");
}


fn dump_token_table(out: &mut Write, dictionary: &HashMap<String, Word>, tokens: &BTreeMap<String, usize>) {
    writeln!(out, "\n\n: tetra_dictionary   # Begin Dictionary Table  2Bytes each");
    for (name, val) in tokens {
        writeln!(out, "0x{:16} 0x{:16} # {} ", val >> 8, val & 0xFF, name);
    }
}


/*
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




fn dump_tetra_header(out: &mut Write) {
    // Constants for the table
    writeln!(out, ":calc DEF_TETRA   {{ 0 }}");
    writeln!(out, ":calc DEF_CALL    {{ 1 }}");
    writeln!(out, ":calc DEF_ADDR    {{ 2 }}");
    writeln!(out, ":calc DEF_LITERAL {{ 3 }}");
}
*/
