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
    id_doubling: bool,
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
        return Font { glyphs: new_glyphs, height: height, id_doubling: true }
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
        let mut max = 0;
        for (c, g) in &self.glyphs {
            max = max.max(g.ref_count);
        }
        let graphw = 45;
        let graphmult: f32 = graphw as f32 / max as f32;
        writeln!(out, "## Font Header").unwrap();

        let mut used = 0;
        for (c, g) in &self.glyphs {
            let s;
            if g.ref_count > 0 {
                let char_id = if self.id_doubling { used * 2 } else { used };
                s = format!(":const G_{} {:3}   # Uses: {}", c, char_id, g.ref_count);
            } else {
                s = format!("#const G_{} {:3}   # Uses: {}", c, "", g.ref_count);
            }
            let w: usize = (g.ref_count as f32 * graphmult) as usize;
            writeln!(out, "{:35} [{}{}]", s, "#".repeat(w),  " ".repeat(graphw - w)).unwrap();
            if g.ref_count > 0 { used += 1 }
        }
        writeln!(out, "# {} of {} glyphs in use", used, self.glyphs.len()).unwrap();
        writeln!(out, "## End Font Header").unwrap();
    }

    pub fn data(&self, out: &mut Write) {
        let mut bytes = 0;
        writeln!(out, "## Font Data\n: glyph_data").unwrap();
        for (c, g) in &self.glyphs {
            if g.ref_count > 0 {
                write!(out, ": glyph_{} {}", c, g.width).unwrap();
                for i in 0..self.height {
                    let b = g.bytes.get(i).unwrap_or(&0);
                    write!(out, " 0x{:02X}", b).unwrap();
                }
                bytes += self.height + 1;
                writeln!(out, " # {} px", g.width).unwrap();
            } else {
                writeln!(out, "# glyph_{} Unused", c).unwrap();
            }
        }
        writeln!(out, "## Font Index\n: glyph_index").unwrap();
        let mut count = 0;
        for (c, g) in &self.glyphs {
            if g.ref_count > 0 {
                write!(out, "TB glyph_{} ", c).unwrap();
                count += 1;
                bytes += 2;
            }
            if count % 8 == 0 { writeln!(out, "").unwrap(); }
        }

        writeln!(out, "\n## End Font Data  {} bytes", bytes).unwrap();
    }

    pub fn code(&self, out: &mut Write) {
        writeln!(out, "## Font Code").unwrap();
        writeln!(out, "\n# Draw glyph with id `n` at location x, y. x and y must be registers\n\
                        :macro draw_glyph x y {{ sprite x y {} }}", self.height).unwrap();
        write!(out, "\n# Sets `i` = the address of glyph `id`\n\
                        # fetch-glyph requires a pair of adjacent registers for reg-hi and reg-lo (v1, v0 or v5, v4 for example)\n\
                        :macro fetch_glyph_macro id reg-hi reg-lo {{ i := long glyph_data i += id ").unwrap();
        if self.id_doubling == false { writeln!(out, "i += id ").unwrap(); }
        writeln!(out, "load reg-lo - reg-hi \
                        i := fetch_glyph_target save reg-hi - reg-lo 0xF0 0x00 : fetch_glyph_target 0x00 0x00 }}"
                ).unwrap();

        writeln!(out, "\n# Creates the `fetch-glyph` function\n\
                        :macro fetch_glyph_impl id reg-hi reg-lo {{ : fetch_glyph fetch_glyph_macro id reg-hi reg-lo ; }}").unwrap();

        writeln!(out, "\n## End Font Code").unwrap();
    }
}
