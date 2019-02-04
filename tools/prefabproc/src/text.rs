use std::collections::BTreeMap;
use std::io::Write;

use crate::font::Font;

/*
    Glyph: a single character
    Word: a string of consecutive characters
    Phrase: a set of one or more symbols
    Symbol: the base unit of a Phrase: glyphs, words, phrase
*/


pub enum Symbol {
    Glyph(char),
    Word(String),
    Color(usize),
}
pub struct Entry {
    name: String,
    contents: Vec<Symbol>,
    ref_count: usize,
}

pub struct Dictionary {
    font: Font,
    entries: BTreeMap<String, Entry>,
}

impl Dictionary {
    pub fn new(font: Font) -> Self {
        return Self { font: font, entries: BTreeMap::new() };
    }

    pub fn insert_from_toml(&mut self, toml_str: &str) {
        let strings: BTreeMap<String, String> = toml::from_str(&toml_str).unwrap();
        for (n, s) in strings {
            self.insert_phrase(&n, &s);
        }
    }

    pub fn insert_phrase(&mut self, name: &str, entry: &str) {
        if let Some(phrase) = self.entries.get_mut(name) {
            phrase.ref_count += 1;
        } else {
            let mut v: Vec<Symbol> = Vec::new();
            for word in name.split(' ') {
                v.push(Symbol::Word(String::from(word)));
                self.insert_word(word);
            }
            let e = Entry { name: String::from(name), contents: v, ref_count: 1 };
            self.entries.insert(String::from(name), e);
        }
    }

    pub fn insert_word(&mut self, name: &str) {
        if let Some(word) = self.entries.get_mut(name) {
            word.ref_count += 1;
        } else {
            let mut v: Vec<Symbol> = Vec::new();
            v.push(Symbol::Word(String::from(name)));
            let e = Entry { name: String::from(name), contents: v, ref_count: 1 };
            self.entries.insert(String::from(name), e);
        }
    }

/*    fn add_text(text_strings: &mut HashMap<String, String>, name: &str, subname: &str, text: Option<&str>) {
        if let Some(text) = &text {
            let mut key: String = String::from(name);
            if subname.chars().count() > 0 {
                key.push('-');
                key.push_str(subname);
            }
            self.insert(key, text.to_string());
        }
    }*/

/*    fn add_text_string(text_strings: &mut HashMap<String, String>, name: &str, subname: &str, text: &Option<String>) { // forgive me rust gods
        if let Some(text) = &text {
            let mut key: String = String::from(name);
            if subname.chars().count() > 0 {
                key.push('-');
                key.push_str(subname);
            }
            self.insert(key, text.to_string());
        }
    }*/


    pub fn header(&self, out: &mut Write) {
        writeln!(out, "## Text Header");
        writeln!(out, "## End Text Header");
    }

    pub fn data(&self, out: &mut Write) {
        writeln!(out, "## Text Data");
        writeln!(out, "## End Text Data");
    }

    pub fn code(&self, out: &mut Write) {
        writeln!(out, "## Text Code");
        writeln!(out, "## End Text Code");
    }

    pub fn process(&mut self) {

    }
/*
    fn process_strings(texts: &HashMap<String, String>, data_dest: &mut Write, _header_dest: &mut Write) {
        let mut words = HashMap::<String, Vec<Symbol>>::new();
        let mut wordvec = Vec::<(String, Vec<Symbol>)>::new();

        for (name, data) in texts {
            println!("INPUT:    {} {:?}\n", name, data);
            let mut phrasevec = Vec::<Symbol>::new();

            for w in data.split_whitespace() {

                let trimmed = w.trim_matches(|c| c == '{' || c == '}');
                if w.chars().count() != trimmed.chars().count() {
                    println!("Brackets: {} {}", &w, &trimmed);
                    break;
                } else {
                    let mut svec = Vec::<Symbol>::new();
                    for letter in w.chars() {
                        svec.push(Symbol::Letter(letter));
                    }
                    if ! words.contains_key(&w.to_string()) {
                        words.insert(w.to_string(), svec.clone());
                        wordvec.push((w.to_string(), svec.clone()));
                    }
                    phrasevec.push(Symbol::Word(w.to_string()));

                }
            }

            if ! words.contains_key(&name.to_string()) {
                words.insert(name.to_string(), phrasevec.clone());
                wordvec.push((name.to_string(), phrasevec.clone()));
            }
        }




        writeln!(data_dest, "### WORDS ###").unwrap();
        for (name, data) in wordvec {
            //println!("{:?} -> {:?}", name, data);
            write!(data_dest, ": word_{} ", name).unwrap();
            for symbol in data.iter() {
                match symbol {
                    Symbol::Letter(c) => { write!(data_dest, ":byte GLYPH_{} ", c).unwrap(); },
                    Symbol::Word(w) => { write!(data_dest, ":byte GLYPH_ESC_WORD tobytes word_{} ", w).unwrap(); },
                }
            }
            writeln!(data_dest, ":byte GLYPH_ESC_END").unwrap();
        }
        writeln!(data_dest, "### END WORDS ###").unwrap();

    }
*/
}
