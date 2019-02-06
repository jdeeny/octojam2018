use std::collections::{BTreeMap, HashSet, VecDeque};
use std::io::Write;

use suffix::SuffixTable;
use argmin::prelude::*;
use argmin::solver::simulatedannealing::{SATempFunc, SimulatedAnnealing};


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
    //input_bytes: usize,
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


    }


    pub fn header(&self, out: &mut Write) {
        self.font.header(out);
        writeln!(out, "## Text Header").unwrap();
        writeln!(out, ":const G_ESC_MASK 0x{:02X}", 0xF0).unwrap();
        writeln!(out, ":const G_ESC_END 0x{:02X}", 0xFF).unwrap();
        writeln!(out, ":const G_ESC_WORD 0x{:02X}", 0xFE).unwrap();
        writeln!(out, ":const G_ESC_SUB 0x{:02X}", 0xFD).unwrap();
        writeln!(out, ":const G_ESC_SPACE 0x{:02X}", 0xFC).unwrap();
        writeln!(out, ":const G_ESC_COLOR_MASK 0x{:02X}", 0xFC).unwrap();
        writeln!(out, ":const G_ESC_COLOR0 0x{:02X}", 0xF8).unwrap();
        writeln!(out, ":const G_ESC_COLOR1 0x{:02X}", 0xF9).unwrap();
        writeln!(out, ":const G_ESC_COLOR2 0x{:02X}", 0xFA).unwrap();
        writeln!(out, ":const G_ESC_COLOR3 0x{:02X}", 0xFB).unwrap();
        writeln!(out, ":const G_ESC_SFX 0x{:02X}", 0xF7).unwrap();
        writeln!(out, ":const G_ESC_PORTRAIT 0x{:02X}", 0xF6).unwrap();
        writeln!(out, "## End Text Header").unwrap();
    }

    pub fn data(&self, out: &mut Write) {
        self.font.data(out);

        let mut to_output: VecDeque<String> = VecDeque::new();
        let mut complete: HashSet<String> = HashSet::new();
        for (n, _) in &self.entries {
            to_output.push_back(n.clone());
        }


        writeln!(out, "## Text Data").unwrap();
        while to_output.len() > 0 {
            let name = to_output.pop_front().unwrap();
            let data = self.entries.get(&name).unwrap();
            let mut ok = true;
            for s in data.contents.iter() {
                if let Symbol::Word(w, _) = s {
                    if !complete.contains(w) {
                        ok = false;
                    }
                }
            }
            if ok {
                let px = data.px.unwrap_or(0);
                write!(out, ": word_{} {{ {} }} ", name, px).unwrap();
                for symbol in data.contents.iter() {
                    match symbol {
                        Symbol::Glyph(c) => { write!(out, "{{ G_{} }} ", c).unwrap(); },
                        Symbol::Word(w, _) => { write!(out, "{{ G_ESC_WORD }} tobytes word_{} ", w).unwrap(); },
                        Symbol::Color(c) => { /*write!(out, "{{ G_ESC_COLOR }} tobytes color_{}", c).unwrap();*/ },
                        Symbol::Sound(sound) => { /*write!(out, "{{ G_ESC_SOUND }} tobytes sfx_{}", sound).unwrap();*/ },
                        Symbol::Portrait(portrait) => { /*write!(out, "{{ G_ESC_PORTRAIT }} tobytes portrait_{} ", portrait).unwrap();*/ },
                        Symbol::Prompt(prompt) => { write!(out, "{{ G_ESC_PROMPT }} tobytes word_{} ", prompt).unwrap(); },
                    }
                }
                writeln!(out, "{{ G_ESC_END }}").unwrap();
                complete.insert(name);
            } else {
                to_output.push_back(name.clone());
            }
        }

        writeln!(out, "## End Text Data  {} bytes", self.data_size()).unwrap();
    }

    pub fn code(&self, out: &mut Write) {
        self.font.code(out);

        writeln!(out, "## Text Code").unwrap();
        writeln!(out, "## End Text Code").unwrap();
    }

    pub fn data_size(&self) -> usize {
        let mut total = 0;
        for (n, e) in self.entries.iter() {
            for symbol in e.contents.iter() {
                match symbol {
                    Symbol::Glyph(c) => { total += 1; },
                    Symbol::Word(w, _) => { total += 3; },
                    Symbol::Color(c) => { /* total += 1; */ },
                    Symbol::Sound(sound) => { /* total += 3; */ },
                    Symbol::Portrait(portrait) => { /* total += 3; */ },
                    Symbol::Prompt(prompt) => { /* total += 3; */},
                }
            }
        }
        return total;
    }

    /* pub fn optimize(in: &Dictionary) -> Result<Dictionary, Error> {
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

        // Define cost function
        //let operator = Rosenbrock::new(1.0, 100.0, lower_bound, upper_bound);
        // definie inital parameter vector
        let init_param: Vec<f64> = vec![1.0, 1.2];
        // Define initial temperature
        let temp = 15.0;
        // Set up simulated annealing solver
        let mut solver = SimulatedAnnealing::new(&operator, init_param, temp)?;

        // Optional: Define temperature function (defaults to `SATempFunc::TemperatureFast`)
        solver.temp_func(SATempFunc::Boltzmann);
        solver.add_logger(ArgminSlogLogger::term());    // Optional: Attach a logger

        /////////////////////////
       // Stopping criteria   //
       /////////////////////////
       solver.set_max_iters(1_000);     // Optional: Set maximum number of iterations (defaults to `std::u64::MAX`)
       solver.set_target_cost(0.0);     // Optional: Set target cost function value (defaults to `std::f64::NEG_INFINITY`)
       solver.stall_best(100);          // Optional: stop if there was no new best solution after 100 iterations
       solver.stall_accepted(100);      // Optional: stop if there was no accepted solution after 100 iterations

       /////////////////////////
       // Reannealing         //
       /////////////////////////
       solver.reannealing_fixed(100);   // Optional: Reanneal after 100 iterations (resets temperature to initial temperature)
       solver.reannealing_accepted(50); // Optional: Reanneal after no accepted solution has been found for 50 iterations
       solver.reannealing_best(80);     // Optional: Start reannealing after no new best solution has been found for 80 iterations

       /////////////////////////
       // Run solver          //
       /////////////////////////
       solver.run()?;
       std::thread::sleep(std::time::Duration::from_secs(1));   // Wait a second (lets the logger flush everything before printing again)
       println!("{:?}", solver.result());
       Ok(())
   }*/


}


/*#[derive(Clone)]
struct DictionaryCost {
    /// Parameter a, usually 1.0
    a: f64,
    /// Parameter b, usually 100.0
    b: f64,
    /// lower bound
    lower_bound: Vec<f64>,
    /// upper bound
    upper_bound: Vec<f64>,
    /// Random number generator. We use a `RefCell` here because `ArgminOperator` requires `self`
    /// to be passed as an immutable reference. `RefCell` gives us interior mutability.
    rng: RefCell<ThreadRng>,
}

impl Rosenbrock {
    /// Constructor
    pub fn new(a: f64, b: f64, lower_bound: Vec<f64>, upper_bound: Vec<f64>) -> Self {
        Rosenbrock {
            a,
            b,
            lower_bound,
            upper_bound,
            rng: RefCell::new(rand::thread_rng()),
        }
    }
}

impl ArgminOperator for Rosenbrock {
    type Parameters = Vec<f64>;
    type OperatorOutput = f64;
    type Hessian = ();

    fn apply(&self, param: &Vec<f64>) -> Result<f64, Error> {
        Ok(rosenbrock(param, self.a, self.b))
    }

    /// This function is called by the annealing function
    fn modify(&self, param: &Vec<f64>, temp: f64) -> Result<Vec<f64>, Error> {
        let mut param_n = param.clone();
        // Perform modifications to a degree proportional to the current temperature `temp`.
        for _ in 0..(temp.floor() as u64 + 1) {
            // Compute random index of the parameter vector using the supplied random number
            // generator.
            let idx = self.rng.borrow_mut().gen_range(0, param.len());

            // Compute random number in [0.01, 0.01].
            let val = 0.01 * self.rng.borrow_mut().gen_range(-1.0, 1.0);

            // modify previous parameter value at random position `idx` by `val`
            let tmp = param[idx] + val;

            // check if bounds are violated. If yes, project onto bound.
            if tmp > self.upper_bound[idx] {
                param_n[idx] = self.upper_bound[idx];
            } else if tmp < self.lower_bound[idx] {
                param_n[idx] = self.lower_bound[idx];
            } else {
                param_n[idx] = param[idx] + val;
            }
        }
        Ok(param_n)
    }
}
*/
