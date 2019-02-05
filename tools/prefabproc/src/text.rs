use std::collections::BTreeMap;
use std::io::Write;

use suffix::SuffixTable;

use crate::font::Font;

/*
    Glyph: a single character
    Word: a string of consecutive characters
    Phrase: a set of one or more symbols
    Symbol: the base unit of a Phrase: glyphs, words, phrase
*/

pub enum Symbol {
    Glyph(char),
    Word(String, usize),
    Color(String),
    Sound(String),
    Portrait(String),
    Prompt(String),
}
pub struct Entry {
    name: String,
    contents: Vec<Symbol>,
    raw: String,
    px: Option<usize>,
    ref_count: usize,
}

pub struct Dictionary {
    font: Font,
    entries: BTreeMap<String, Entry>,
}

impl Dictionary {
    pub fn new(mut font: Font) -> Self {
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
            self.font.mark_used(entry);
        } else {
            let mut v: Vec<Symbol> = Vec::new();
            for w in entry.split_whitespace() {
                let trimmed = w.trim_matches(|c| c == '{' || c == '}');
                if w.chars().count() != trimmed.chars().count() {
                    println!("Brackets: {} {}", &w, &trimmed);
                    if let Some(n) = trimmed.rfind("portrait:") {
                        let name = trimmed.split(':').last().unwrap();
                        println!("Portrait: {}", &name);
                        v.push(Symbol::Portrait(String::from(name)));
                    } else if let Some(n) = trimmed.rfind("sound:") {
                        let name = trimmed.split(':').last().unwrap();
                        println!("Sound: {}", &name);
                        v.push(Symbol::Sound(String::from(name)));
                    } else if let Some(n) = trimmed.rfind("color:") {
                        let name = trimmed.split(':').last().unwrap();
                        println!("Color: {}", &name);
                        v.push(Symbol::Color(String::from(name)));
                    }
                } else if w.chars().count() > 1 {
                    v.push(Symbol::Word(String::from(w), self.font.get_width(w)));
                } else {
                    v.push(Symbol::Glyph(w.chars().next().unwrap()));
                }
            }
            if v.len() == 1 {
                self.insert_word(name, entry);
            } else {
                // put all the sub-words in the dictionary
                for sym in &v {
                    if let Symbol::Word(w, _) = sym {
                        self.insert_word(w, w)
                    }
                }

                let e = Entry { name: String::from(name), raw: String::from(entry), contents: v, ref_count: 1, px: None };
                self.entries.insert(String::from(name), e);
            }
        }
    }

    pub fn insert_word(&mut self, name: &str, data: &str) {
        self.font.mark_used(data);
        if let Some(word) = self.entries.get_mut(name) {
            word.ref_count += 1;
        } else {
            let mut v: Vec<Symbol> = Vec::new();
            if data.chars().count() > 1 {
                for c in data.chars() {
                    v.push(Symbol::Glyph(c));
                }
                //v.push(Symbol::Word(String::from(name), self.font.get_width(name)));
            } else {
                v.push(Symbol::Glyph(data.chars().next().unwrap()));
            }
            let e = Entry { name: String::from(name), raw: String::from(data), contents: v, ref_count: 1, px: Some(self.font.get_width(data)) };
            self.entries.insert(String::from(name), e);
        }
    }


    pub fn process(&mut self) {
        let mut corpus: Vec<SuffixTable> = Vec::new();// String::from("ABC ABCD BCDE DEF");
        let mut combined = String::new();
        for (_k, e) in &self.entries {
            corpus.push(SuffixTable::new(e.raw.clone()));
            combined.push_str(&e.raw);
            combined.push('~');
        }
        let combined_suffix = SuffixTable::new(&combined);
        //println!("{:?}", corpus);
        //println!("{:?}", combined_suffix);


    }


    pub fn header(&self, out: &mut Write) {
        self.font.header(out);
        writeln!(out, "## Text Header").unwrap();
        writeln!(out, ":const G_ESC_MASK {:02X}", 0xF8).unwrap();
        writeln!(out, ":const G_ESC_END {:02X}", 0xFF).unwrap();
        writeln!(out, ":const G_ESC_WORD {:02X}", 0xFE).unwrap();
        writeln!(out, ":const G_ESC_SUB {:02X}", 0xFD).unwrap();
        writeln!(out, ":const G_ESC_SPACE {:02X}", 0xFC).unwrap();
        writeln!(out, ":const G_ESC_COLOR_MASK {:02X}", 0xFC).unwrap();
        writeln!(out, ":const G_ESC_COLOR0 {:02X}", 0xF8).unwrap();
        writeln!(out, ":const G_ESC_COLOR1 {:02X}", 0xF9).unwrap();
        writeln!(out, ":const G_ESC_COLOR2 {:02X}", 0xFA).unwrap();
        writeln!(out, ":const G_ESC_COLOR3 {:02X}", 0xFB).unwrap();
        writeln!(out, "## End Text Header").unwrap();
    }

    pub fn data(&self, out: &mut Write) {
        self.font.data(out);

        writeln!(out, "## Text Data").unwrap();
        for (name, data) in &self.entries {
            //println!("{:?} -> {:?}", name, data);
            let px = data.px.unwrap_or(0);
            write!(out, ": word_{} {{ {} }} ", name, px).unwrap();
            for symbol in data.contents.iter() {
                match symbol {
                    Symbol::Glyph(c) => { write!(out, "{{ G_{} }} ", c).unwrap(); },
                    Symbol::Word(w, _) => { write!(out, "{{ G_ESC_WORD }} tobytes word_{} ", w).unwrap(); },
                    Symbol::Color(c) => { write!(out, "{{ G_ESC_COLOR }} tobytes color_{}", c).unwrap(); },
                    Symbol::Sound(sound) => { write!(out, "{{ G_ESC_SOUND }} tobytes sfx_{}", sound).unwrap(); },
                    Symbol::Portrait(portrait) => { write!(out, "{{ G_ESC_PORTRAIT }} tobytes portrait_{} ", portrait).unwrap(); },
                    Symbol::Prompt(prompt) => { write!(out, "{{ G_ESC_PROMPT }} tobytes word_{} ", prompt).unwrap(); },
                }
            }
            writeln!(out, "{{ G_ESC_END }}").unwrap();
        }
        writeln!(out, "## End Text Data").unwrap();
    }

    pub fn code(&self, out: &mut Write) {
        self.font.code(out);

        writeln!(out, "## Text Code").unwrap();
        writeln!(out, "## End Text Code").unwrap();
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
