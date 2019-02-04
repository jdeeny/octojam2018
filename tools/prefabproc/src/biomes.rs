use std::collections::BTreeMap;
use std::io::Write;

use crate::text::Dictionary;

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
    biomes: BTreeMap<String, Biome>,
    sorted_biomes: Vec<(String, Biome)>,
}

impl Biomes {
    pub fn from_toml(toml_str: &str) -> Self {
        let biomes: BTreeMap<String, Biome> = toml::from_str(toml_str).unwrap();
        let mut sorted_biomes: Vec<(String, Biome)> = Vec::new();
        for b in &biomes { sorted_biomes.push((b.0.clone(), b.1.clone())); }
        sorted_biomes.sort_by_key(|x| x.1.order );
        return Self { biomes: biomes, sorted_biomes: sorted_biomes };
    }

    pub fn header(&self, out: &mut Write) {
        writeln!(out, "## Biome Header").unwrap();
        writeln!(out, ":const BIOME_COUNT {}", self.biomes.len()).unwrap();
        writeln!(out, ":const BIOME_LAST {}", self.biomes.len() - 1).unwrap();
        writeln!(out, "## End Biome Header").unwrap();
    }

    pub fn data(&self, out: &mut Write) {
        writeln!(out, "## Biome Data").unwrap();
        writeln!(out, ": biome_data").unwrap();
        let mut count = 0;
        for (name, data) in self.sorted_biomes.iter() {
            for level in 0..data.levels {
                count += 1;
                let narration: String;
                if level > 0 {
                    narration = String::from("none");
                } else if let Some(_n) = data.narration.clone() {
                    narration = name.to_string();
                } else {
                    narration = String::from("none");
                }
                let level_name = format!("{}{}", &name, &level);
                let _level_display = format!("{}  -  {}", name, level + 1);
    //            text_strings.insert(format!("word_Biome_Name_{}", biome_name), biome_display);
                write!(out, ": biome_{} tobytes word_Biome_Name_{} tobytes word_narration_{} tobytes enemyset_{} 0 0\n", level_name, level_name, narration, name).unwrap();

            }
            // tileset - unused for now
            // create a word for the name
            // create an enemy set
            // create a narration event
            // create a treasure set
        }


        writeln!(out, "## End Biome Data").unwrap();
    }

    pub fn code(&self, out: &mut Write) {
        writeln!(out, "## Biome Code").unwrap();
        writeln!(out, "## End Biome Code").unwrap();
    }

    pub fn process_strings(&self, dict: &mut Dictionary) {
        for (name, data) in self.biomes.iter() {
            if let Some(narration) = &data.narration {
                let key = format!("narration_{}", &name);
                dict.insert_phrase(&key, narration);
            }
        }

        for (name, data) in self.biomes.iter() {
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






/*


fn process_biomes(biomes: &BTreeMap<String, Biome>, _text_strings: &mut HashMap<String, String>, data_out: &mut Write, header_out: &mut Write) {

    writeln!(data_out, "### Biomes (Level List) ###").unwrap();
    writeln!(data_out, ":const word_narration_none 0").unwrap();
    writeln!(data_out, ": level_state 0").unwrap();



    writeln!(header_out, ":const level_count {}", count).unwrap();
    writeln!(header_out, ":const level_last {}", count - 1).unwrap();
}




fn biomes_make_strings(biomes: &BTreeMap<String, Biome>, text_strings: &mut HashMap<String, String>, _data_out: &mut Write, _header_out: &mut Write) {
    println!("Processing Biomes");

}
*/
