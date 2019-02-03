use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct Glyph {
    name: String,
    data: String,
    bytes: Vec<u8>,
    width: usize,
}

pub struct Font {
    glyphs: BTreeMap<String, Glyph>,
    height: usize,
}

impl Font {
    pub fn from_toml(toml_str: &str) -> Self {
        let glyphs: BTreeMap<String, String> = toml::from_str(toml_str).unwrap();
        let mut height = 0;
        let mut new_glyphs: BTreeMap<String, Glyph> = BTreeMap::new();
        for (name, g) in glyphs {
            let mut width = 0;
            let mut gheight = 0;
            let mut bytes = Vec::<u8>::new();
            for l in g.split('\n') {
                let lwidth = l.rfind('X').unwrap_or(0);
                width = std::cmp::max(width, lwidth);
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
            new_glyphs.insert(name.clone(), Glyph{name: name, data: g, bytes: bytes, width: width });
            height = std::cmp::max(height, gheight);
        }
        return Font { glyphs: new_glyphs, height: height }
    }
}
