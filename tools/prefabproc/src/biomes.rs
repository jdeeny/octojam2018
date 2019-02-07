use std::collections::BTreeMap;
use std::io::Write;

use crate::text::Dictionary;
use crate::datatable::*;

#[derive(Debug, Deserialize, Clone)]
pub struct Biome {
    name: String,
    order: usize,
    tileset: String,
    levels: usize,
    enemies: Vec<String>,
    treasure: Vec<String>,
    narration: Option<String>,
}

pub struct Biomes {
    biomes: DataTable,
    sorted_biomes: Vec<(String, Biome)>,
}

impl Biomes {
    pub fn from_toml(toml_str: &str) -> Self {
        let cols = vec!(
            ("name".into(), DataKind::Label),
            ("narration".into(), DataKind::Label),
            ("enemyset".into(), DataKind::Label),
            ("treasureset".into(), DataKind::Label),
        );

        let biomes: BTreeMap<String, Biome> = toml::from_str(toml_str).unwrap();
        let mut dt = DataTable::new("Biomes", &cols);

        let mut sorted_biomes: Vec<(String, Biome)> = Vec::new();
        for b in &biomes { sorted_biomes.push((b.0.clone(), b.1.clone())); }
        sorted_biomes.sort_by_key(|x| x.1.order );

        for (n, b) in &sorted_biomes {
            let mut narration_str = format!("word_narration_{}", &n);
            for level in 0 .. b.levels {
                //let name_str = format!("word_Biome_Name_{}", b.name);
                let enemyset = format!("word_narration_{}", &n);
                let level_name = format!("{}{}", &b.name.replace(" ", "_"), level);
                let name_str = format!("word_Biome_Name_{}{}", &n, level);
                let treasureset = format!("treasure_list_{}", &n);
                if level > 0 { narration_str = String::from("BIOME_NARRATION_NONE") }
                let row: Vec<Data> = vec!(Data::Label(name_str), Data::Label(narration_str.clone()), Data::Label(enemyset), Data::Label(treasureset));
                dt.add(&level_name, row);
            }
        }

        return Self { biomes: dt, sorted_biomes: sorted_biomes };
    }

    pub fn header(&self, out: &mut Write) {
        writeln!(out, "\n## Biome Constants").unwrap();
        writeln!(out, ":const BIOME_NARRATION_NONE 0").unwrap();
        writeln!(out, "{}", self.biomes.octo_header()).unwrap();
    }

    pub fn data(&self, out: &mut Write) {
        writeln!(out, "\n## Biome Data").unwrap();
        for (n, b) in &self.sorted_biomes {
            write!(out, ": treasure_list_{} ", n).unwrap();
            for t in &b.treasure {
                write!(out, "TB treasure_{} ", t.replace(" ", "_")).unwrap();
            }
            writeln!(out, "").unwrap();
        }
        writeln!(out, "{}", self.biomes.octo_data()).unwrap();
        writeln!(out, "## End Biome Data").unwrap();
    }
/*        writeln!(out, "## Biome Data").unwrap();
        for (name, data) in self.biomes.iter() {
            write!(out, ": enemyset_{} {} ", name, data.enemies.len()).unwrap();
            for e in &data.enemies {
                write!(out, "tobytes enemy_{} ", &e).unwrap();
            }
            writeln!(out, "").unwrap();
        }

        writeln!(out, ": biome_data").unwrap();
        for (name, data) in self.sorted_biomes.iter() {
            for level in 0..data.levels {
                let narration: String;
                if level > 0 {
                    narration = String::from("BIOME_NARRATION_NONE");
                } else if let Some(n) = data.narration.clone() {
                    narration = format!("word_narration_{}", name);
                } else {
                    narration = String::from("BIOME_NARRATION_NONE");
                }
                let level_name = format!("{}{}", &name, &level);
                let _level_display = format!("{}  -  {}", name, level + 1);
                write!(out, ": biome_{} tobytes word_Biome_Name_{} tobytes {} tobytes enemyset_{} 0 0\n", level_name, level_name, narration, name).unwrap();
            }
        }
        writeln!(out, "## End Biome Data").unwrap();*/

    pub fn code(&self, out: &mut Write) {
        writeln!(out, "## Biome Code").unwrap();
        write!(out, "{}", self.biomes.octo_code()).unwrap();
        writeln!(out, "## End Biome Code").unwrap();
    }

    pub fn process_strings(&self, dict: &mut Dictionary) {
        for (name, data) in self.sorted_biomes.iter() {
            if let Some(narration) = &data.narration {
                let key = format!("narration_{}", &name);
                dict.insert_phrase(&key, narration);
            }
        }

        for (name, data) in self.sorted_biomes.iter() {
            for level in 0..data.levels {
                let biome_name = format!("{}{}", &name, level);
                let biome_display = format!("{}  -  {}", &data.name, level + 1);
                dict.insert_phrase(&format!("Biome_Name_{}", biome_name), &biome_display);
            }
        }
    }

    pub fn process(&mut self) {

    }


}
