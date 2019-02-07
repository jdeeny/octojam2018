use std::collections::BTreeMap;
use std::io::Write;
use toml::Value;

use crate::text::Dictionary;
use crate::datatable::*;

#[derive(Debug, Deserialize)]
struct TreasureItem {
    name: String,
    art: String,
    active: Option<Vec<Value>>,
    desc: Option<String>,
}

pub struct Treasure {
    treasure: DataTable,
    input: BTreeMap<String, TreasureItem>,
}

impl Treasure {
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

        let treasure: BTreeMap<String, TreasureItem> = toml::from_str(toml_str).unwrap();
        let mut dt = DataTable::new("Treasure", &cols);

        for (n, b) in &treasure {
            let row: Vec<Data> = vec!(Data::Byte(0), Data::Byte(0), Data::Byte(0),
                Data::Label(format!("SPR_{}", b.art)), Data::Label(format!("SPR_portrait_{}", b.art)),
                Data::Label(format!("word_{}-desc", n)),
                Data::Label(format!("ai_treasure")),
                Data::Label(format!("word_{}-name", n)),
            );
            dt.add(&format!("treasure_{}", n), row);
        }

        return Self { treasure: dt, input: treasure };
    }

    pub fn header(&self, out: &mut Write) {
        writeln!(out, "\n## Treasure Constants").unwrap();
        writeln!(out, "{}", self.treasure.octo_header()).unwrap();
    }

    pub fn data(&self, out: &mut Write) {
        writeln!(out, "\n## Treasure Data").unwrap();
        writeln!(out, "{}", self.treasure.octo_data()).unwrap();
        writeln!(out, "## End Treasure Data").unwrap();

        /*writeln!(out, "## Treasure Data").unwrap();
        for (name, data) in self.items.iter() {
            write!(out, ": treaure_{} 0 0 0 0 0 0 0 0 0 0 0 0", name).unwrap();
        }
        writeln!(out, "## End Treasure Data").unwrap();*/
    }

    pub fn code(&self, out: &mut Write) {
    }

    pub fn process(&mut self) {

    }

    pub fn process_strings(&self, dict: &mut Dictionary) {
        for (name, data) in self.input.iter() {
            dict.insert_phrase(&format!("{}-name", &name), &data.name);
            if let Some(desc) = &data.desc {
                dict.insert_phrase(&format!("{}-desc", name), &desc);
            }
        }
    }

}


/*

fn process_treasure_lists(biomes: &BTreeMap<String, Biome>, _text_strings: &mut HashMap<String, String>, data_out: &mut Write, _header_out: &mut Write) {
    println!("Processing Biome Treasure Lists");
    writeln!(data_out, "### Biome Treasure Lists ###").unwrap();
    for (name, data) in biomes.iter() {
        write!(data_out, ": treasure_list_{} {} ", name, data.treasure.len()).unwrap();
        for e in &data.treasure {
            write!(data_out, "tobytes treasure_{} ", e).unwrap();
        }
        writeln!(data_out, "").unwrap();
    }
    writeln!(data_out, "  ### End Treasure Lists ###\n\n").unwrap();
}


fn process_treasure(treasure: &BTreeMap<String, Treasure>, text_strings: &mut HashMap<String, String>, data_out: &mut Write, header_out: &mut Write) {
    println!("Processing Treasure");
    writeln!(data_out, "### Treasure Prefabs ###").unwrap();
    for (name, data) in treasure.iter() {
        let x = 0;
        let y = 0;
        let flags = 0;
        let ai = String::from("treasure");
        let sprite = data.art.clone();

        add_text(text_strings, name, "name", Some(name));
        add_text_string(text_strings, name, "desc", &data.desc);

        write!(data_out, ": treasure_{} {} {} {} tobytes SPR_{} tobytes SPR_portrait_{} tobytes word_{}-desc tobytes ai_{} tobytes word_{}-name", name, x, y, flags, sprite, sprite, name, ai, name);

        //write!(data_out, ": treasure_{} {} {} {} tobytes SPR_{} tobytes ai_{}", name, x, y, flags, sprite, ai);
        writeln!(data_out, " # '{}'", name);
    }
    writeln!(data_out, "0xFF\n### End Treasure Data ###\n\n").unwrap();



    writeln!(header_out, ":const treasure_prefab_count {}", treasure.keys().count());

    writeln!(header_out, ":const treasure_offset_x 0\n");
    writeln!(header_out, ":const treasure_offset_y 1\n");
    writeln!(header_out, ":const treasure_offset_flags 2\n");
    writeln!(header_out, ":const treasure_offset_sprite 3\n");
    writeln!(header_out, ":const treasure_offset_portrait 5\n");
    writeln!(header_out, ":const treasure_offset_desc 7\n");
    writeln!(header_out, ":const treasure_offset_ai 9\n");
    writeln!(header_out, ":const treasure_offset_name 11");
    writeln!(header_out, ":const treasure_table_bytes 13");
    writeln!(data_out, "\n: treasure_ptrs\n");
    for  (name, data) in treasure.iter() {
        writeln!(data_out, "    tobytes treasure_{}", name);
    }




}

fn treasure_make_strings(treasure: &BTreeMap<String, Treasure>, text_strings: &mut HashMap<String, String>, data_out: &mut Write, header_out: &mut Write) {

}
*/
