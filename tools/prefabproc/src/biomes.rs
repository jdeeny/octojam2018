use std::collections::BTreeMap;
use std::io::Write;

#[derive(Debug, Deserialize)]
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
}

impl Biomes {
    pub fn from_toml(toml_str: &str) -> Self {
        let biomes: BTreeMap<String, Biome> = toml::from_str(toml_str).unwrap();
        return Self { biomes: biomes };
    }

    pub fn header(&self, out: &Write) {

    }

    pub fn data(&self, out: &Write) {

    }


    pub fn process(&mut self) {

    }

}






/*


fn process_biomes(biomes: &BTreeMap<String, Biome>, _text_strings: &mut HashMap<String, String>, data_out: &mut Write, header_out: &mut Write) {
    println!("Processing Biomes");
    for (name, data) in biomes.iter() {
        println!("{} -> {:?}", &name, data);
        if let Some(_narration) = &data.narration {
            let _key = format!("narration_{}", &name);
//            text_strings.insert(key, narration.clone());
        }
    }

    writeln!(data_out, "### Biomes (Level List) ###").unwrap();
    writeln!(data_out, ":const word_narration_none 0").unwrap();
    writeln!(data_out, ": level_state 0").unwrap();
    writeln!(data_out, ": biome_data").unwrap();

    let mut sorted_biomes: Vec<(&String, &Biome)> = biomes.iter().collect();
    sorted_biomes.sort_by_key(|x| x.1.order );


    let mut count = 0;
    for (name, data) in sorted_biomes.iter() {
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
            write!(data_out, ": biome_{} tobytes word_Biome_Name_{} tobytes word_narration_{} tobytes enemyset_{} 0 0\n", level_name, level_name, narration, name).unwrap();

        }
        // tileset - unused for now
        // create a word for the name
        // create an enemy set
        // create a narration event
        // create a treasure set
    }
    writeln!(data_out, "0xFF   ### End Biome Data ###\n\n").unwrap();

    writeln!(header_out, ":const level_count {}", count).unwrap();
    writeln!(header_out, ":const level_last {}", count - 1).unwrap();
}




fn biomes_make_strings(biomes: &BTreeMap<String, Biome>, text_strings: &mut HashMap<String, String>, _data_out: &mut Write, _header_out: &mut Write) {
    println!("Processing Biomes");
    for (name, data) in biomes.iter() {
        if let Some(narration) = &data.narration {
            let key = format!("narration_{}", &name);
            text_strings.insert(key, narration.clone());
        }
    }

    for (name, data) in biomes.iter() {
        for level in 0..data.levels {
            let biome_name = format!("{}{}", &name, level);
            let biome_display = format!("{}  -  {}", &data.name, level + 1);
            text_strings.insert(format!("Biome_Name_{}", biome_name), biome_display);
        }
    }
}
*/
