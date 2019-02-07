use std::collections::BTreeMap;
use std::io::Write;
use toml::Value;

use crate::text::Dictionary;
use crate::datatable::*;

#[derive(Debug, Deserialize)]
struct Enemy {
    name: String,
    art: String,
    hp: Option<usize>,
    ac: Option<usize>,
    ammo: Option<usize>,
    reload: Option<usize>,
    ai: String,
    speed: Option<usize>,
    treasure: Option<Vec<Value>>,
    attacks: Option<Vec<Value>>,
    desc: Option<String>,
}

pub struct Enemies {
    enemies: DataTable,
    input: BTreeMap<String, Enemy>,
}

impl Enemies {
    pub fn from_toml(toml_str: &str) -> Self {
        let cols = vec!(
            ("x".into(), DataKind::Byte),
            ("y".into(), DataKind::Byte),
            ("flags".into(), DataKind::Label),
            ("sprite".into(), DataKind::Label),
            ("portrait".into(), DataKind::Label),
            ("desc".into(), DataKind::Label),
            ("ai".into(), DataKind::Label),
            ("name".into(), DataKind::Label),
        );

        let enemies: BTreeMap<String, Enemy> = toml::from_str(toml_str).unwrap();
        let mut dt = DataTable::new("Enemies", &cols);

        for (n, b) in &enemies {
            let row: Vec<Data> = vec!(Data::Byte(0), Data::Byte(0), Data::Byte(0),
                Data::Label(format!("SPR_{}", &b.art)), Data::Label(format!("SPR_portrait_{}", &b.art)),
                Data::Label(format!("word_{}-desc", &n)),
                Data::Label(format!("ai_{}", &b.ai)),
                Data::Label(format!("word_{}-name", &n)),
            );
            dt.add(&n, row);
        }

        return Self { enemies: dt, input: enemies };
    }

    pub fn header(&self, out: &mut Write) {
        writeln!(out, "\n## Enemy Constants").unwrap();
        writeln!(out, "{}", self.enemies.octo_header()).unwrap();

        writeln!(out, "## Enemy Header").unwrap();
        writeln!(out, ":const ENEMY_COUNT {}", self.enemies.len()).unwrap();
        writeln!(out, ":const ENEMY_LAST {}", self.enemies.len() - 1).unwrap();
        writeln!(out, ":const ENEMY.X 0").unwrap();
        writeln!(out, ":const ENEMY.Y 1").unwrap();
        writeln!(out, ":const ENEMY.FLAGS 2").unwrap();
        writeln!(out, ":const ENEMY.SPRITE 3").unwrap();
        writeln!(out, ":const ENEMY.PORTRAIT 5").unwrap();
        writeln!(out, ":const ENEMY.DESC 7").unwrap();
        writeln!(out, ":const ENEMY.AI 9").unwrap();
        writeln!(out, ":const ENEMY.NAME 11").unwrap();
        writeln!(out, ":const ENEMY_BYTES 13").unwrap();
        writeln!(out, "## End Enemy Header").unwrap();

    }

    pub fn data(&self, out: &mut Write) {
    /*    writeln!(out, "## Enemy Data").unwrap();
        writeln!(out, "### Enemy Prefabs ###").unwrap();
        for (name, data) in self.input.iter() {
            let x = 0;
            let y = 0;
            let flags = 0;
            let ai = &data.ai.clone();
            let sprite = &data.art.clone();

            write!(out, ": enemy_{} {} {} {} tobytes SPR_{} tobytes SPR_portrait_{} tobytes word_{}-desc tobytes ai_{} tobytes word_{}-name", name, x, y, flags, sprite, sprite, name, ai, name).unwrap();
            writeln!(out, " # '{}'", name).unwrap();
        }
        writeln!(out, "0xFF\n### End Enemy Prefabs ###\n\n").unwrap();

        writeln!(out, "\n: enemy_ptrs\n").unwrap();
        for  (name, data) in self.input.iter() {
            writeln!(out, "    tobytes enemy_{}", name).unwrap();
        }

        writeln!(out, "## End Enemy Data").unwrap();

*/
        writeln!(out, "\n## Enemy Data").unwrap();
        writeln!(out, "{}", self.enemies.octo_data()).unwrap();
        writeln!(out, "## End Enemy Data").unwrap();

    }

    pub fn code(&self, out: &mut Write) {
        writeln!(out, "## Enemy Code").unwrap();
        writeln!(out, "## End Enemy Code").unwrap();
    }

    pub fn process(&mut self) {

    }

    pub fn process_strings(&self, dict: &mut Dictionary) {
        for (name, data) in self.input.iter() {
            dict.insert_phrase(&format!("{}-name", name), &data.name);
            if let Some(desc) = &data.desc {
                dict.insert_phrase(&format!("{}-desc", name), &desc);
            }
        }
    }
}






/*




fn enemies_make_strings(enemies: &BTreeMap<String, Enemy>, text_strings: &mut HashMap<String, String>, _out: &mut Write, _header_out: &mut Write) {
    for (name, data) in enemies.iter() {
        add_text(text_strings, name, "name", Some(&data.name));
        //add_text(text_strings, name, "name", Some(name));
        add_text_string(text_strings, name, "desc", &data.desc);

    }
}


fn process_enemies(enemies: &BTreeMap<String, Enemy>, _text_strings: &mut HashMap<String, String>, out: &mut Write, header_out: &mut Write) {
    println!("Processing Enemies");
    writeln!(out, "### Enemy Prefabs ###").unwrap();
    for (name, data) in enemies.iter() {
        let x = 0;
        let y = 0;
        let flags = 0;
        let mut ai = String::from("default");
        if let Some(a) = &data.ai { ai = a.clone();}
        let sprite = data.art.clone();

        write!(out, ": enemy_{} {} {} {} tobytes SPR_{} tobytes SPR_portrait_{} tobytes word_{}-desc tobytes ai_{} tobytes word_{}-name", name, x, y, flags, sprite, sprite, name, ai, name);
        writeln!(out, " # '{}'", name);
    }
    writeln!(out, "0xFF\n### End Enemy Prefabs ###\n\n");

    writeln!(header_out, ":const enemy_prefab_count {}", enemies.keys().count());
    writeln!(header_out, ":const enemy_offset_x 0\n");
    writeln!(header_out, ":const enemy_offset_y 1\n");
    writeln!(header_out, ":const enemy_offset_flags 2\n");
    writeln!(header_out, ":const enemy_offset_sprite 3\n");
    writeln!(header_out, ":const enemy_offset_portrait 5\n");
    writeln!(header_out, ":const enemy_offset_desc 7\n");
    writeln!(header_out, ":const enemy_offset_ai 9\n");
    writeln!(header_out, ":const enemy_offset_name 11");
    writeln!(header_out, ":const enemy_table_bytes 13");
    writeln!(out, "\n: enemy_ptrs\n");
    for  (name, data) in enemies.iter() {
        writeln!(out, "    tobytes enemy_{}", name);
    }

}
*/
