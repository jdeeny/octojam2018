use std::collections::BTreeMap;
use std::io::Write;


#[derive(Clone)]
pub struct Glyph {
    name: String,
    data: String,
    bytes: Vec<u8>,
    width: usize,
    ref_count: usize,
}

#[derive(Clone)]
pub struct Font {
    glyphs: BTreeMap<char, Glyph>,
    height: usize,
}

impl Font {
    pub fn from_toml(toml_str: &str) -> Self {
        let glyphs: BTreeMap<String, String> = toml::from_str(toml_str).unwrap();
        let mut height = 0;
        let mut new_glyphs: BTreeMap<char, Glyph> = BTreeMap::new();
        for (name, g) in glyphs {
            let mut width = 0;
            let mut gheight = 0;
            let mut bytes = Vec::<u8>::new();
            for l in g.split('\n') {
                if let Some(loc) = l.rfind('X') {
                    width = std::cmp::max(width, loc + 1);
                }

                gheight += 1;
                let mut b = 0;
                let mut m = 0x80;
                for c in l.chars() {
                    if c == 'X' {
                        b |= m;
                        m >>= 1;
                    }
                }
                bytes.push(b);
            }
            width += 1; // letter spacing
            new_glyphs.insert(name.chars().next().unwrap(), Glyph{name: name.to_string(), data: g, bytes: bytes, width: width, ref_count: 0 });
            height = std::cmp::max(height, gheight);
        }
        return Font { glyphs: new_glyphs, height: height }
    }

    pub fn mark_used(&mut self, word: &str) {
        for c in word.chars() {
            if let Some(g) = self.glyphs.get_mut(&c) {
                g.ref_count += 1;
            } else {
                println!("Glyph Not Found '{}'", c);
            }
        }
    }

    pub fn get_width(&self, word: &str) -> usize {
        let mut w = 0;
        for c in word.chars() {
            if let Some(g) = self.glyphs.get(&c) {
                w += g.width;
            } else {
                println!("Glyph Not Found '{}'", c);
            }
        }
        return w;
    }

    pub fn header(&self, out: &mut Write) {
        let mut count = 0;
        writeln!(out, "## Font Header").unwrap();
        for (c, g) in &self.glyphs {
            writeln!(out, ":macro GLYPH_{} {{ :byte {} }} # Uses: {}", c, count, g.ref_count).unwrap();
            count += 1;
        }
        writeln!(out, "## End Font Header").unwrap();
    }

    pub fn data(&self, out: &mut Write) {
        writeln!(out, "## Font Data\n : glyph_data").unwrap();
        for (c, g) in &self.glyphs {
            write!(out, ": glyph_{} {}", c, g.width).unwrap();
            for i in 0..self.height {
                let b = g.bytes.get(i).unwrap_or(&0);
                write!(out, " 0x{:02X}", b).unwrap();
            }
            writeln!(out, " # {} px", g.width).unwrap();
        }
        writeln!(out, "## End Font Data").unwrap();
    }

    pub fn code(&self, out: &mut Write) {
        writeln!(out, "## Font Code").unwrap();
        writeln!(out, "## End Font Code").unwrap();
    }
}
