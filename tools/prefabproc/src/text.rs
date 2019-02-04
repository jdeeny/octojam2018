use std::collections::BTreeMap;
use std::io::Write;
use toml::Value;

#[derive(Debug, Clone)]
pub enum Symbol {
    Letter(char),
    Word(String),
}

#[derive(Debug, Deserialize)]
pub struct Entry {
    name: String,
    contents: Vec<String>,
}

pub struct Dictionary {
    entries: BTreeMap<String, Entry>,
}

impl Dictionary {
    pub fn new() -> Self {
        return Self { entries: BTreeMap::new() };
    }

    pub fn insert(&mut self, name: &str, entry: &str) {
        let mut contents = Vec::new();
        contents.push(String::from(entry));
        let entry = Entry{name: String::from(name), contents: contents };
        self.entries.insert(String::from(name), entry);
    }

    pub fn header(&self, out: &Write) {

    }

    pub fn data(&self, out: &Write) {

    }


    pub fn process(&mut self) {

    }

}
/*

fn add_text(text_strings: &mut HashMap<String, String>, name: &str, subname: &str, text: Option<&str>) {
    if let Some(text) = &text {
        let mut key: String = String::from(name);
        if subname.chars().count() > 0 {
            key.push('-');
            key.push_str(subname);
        }
        text_strings.insert(key, text.to_string());
    }
}



fn add_text_string(text_strings: &mut HashMap<String, String>, name: &str, subname: &str, text: &Option<String>) { // forgive me rust gods
    if let Some(text) = &text {
        let mut key: String = String::from(name);
        if subname.chars().count() > 0 {
            key.push('-');
            key.push_str(subname);
        }
        text_strings.insert(key, text.to_string());
    }
}




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
