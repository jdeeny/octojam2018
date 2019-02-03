
use std::collections::{ HashMap, BTreeMap };
use std::fs::File;
use std::io::{Read,Write};
use toml::{Value};

#[macro_use]
extern crate serde_derive;



#[derive(Debug, Deserialize)]
struct Attack {
    range: Option<f64>,
    dmg: Option<f64>,
    Hit: Option<String>,
    Miss: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Biome {
    name: String,
    order: usize,
    tileset: String,
    levels: usize,
    enemies: Vec<String>,
    treasure: Vec<String>,
    narration: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Enemy {
    name: String,
    art: String,
    hp: Option<usize>,
    ac: Option<usize>,
    ammo: Option<usize>,
    reload: Option<usize>,
    ai: Option<String>,
    speed: Option<usize>,
    treasure: Option<Vec<Value>>,
    attacks: Option<Vec<Value>>,
    desc: Option<String>,
}


#[derive(Debug, Deserialize)]
struct Treasure {
    name: String,
    art: String,
    active: Option<Vec<Value>>,
    desc: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Weapon {
    Type: Option<String>,
    Damage: Option<String>,
    AttackDescription: Option<Vec<String>>,

}


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



fn process_attacks(attacks: &BTreeMap<String, Attack>, text_strings: &mut HashMap<String, String>, data_out: &mut Write) {
    println!("Processing Attacks");

    writeln!(data_out, "### Attack Data ###").unwrap();
    for (name, data) in attacks.iter() {
        if let Some(hitmsg) = &data.Hit {
            add_text(text_strings, name, "hitmsg", Some(hitmsg));
        }
        if let Some(missmsg) = &data.Miss {
            add_text(text_strings, name, "missmsg", Some(missmsg));
        }
    }
    writeln!(data_out, "### End Attack Data ###\n\n").unwrap();


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

fn process_enemy_lists(biomes: &BTreeMap<String, Biome>, _text_strings: &mut HashMap<String, String>, data_out: &mut Write, _header_out: &mut Write) {
    println!("Processing Biome Enemy Lists");
    writeln!(data_out, "### Biome Enemy Lists ###").unwrap();
    for (name, data) in biomes.iter() {
        write!(data_out, ": enemyset_{} {} ", name, data.enemies.len()).unwrap();
        for e in &data.enemies {
            write!(data_out, "tobytes enemy_{} ", &e).unwrap();
        }
        writeln!(data_out, "").unwrap();
    }
    writeln!(data_out, "  ### End Enemy Lists ###\n\n").unwrap();
}

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



fn treasure_make_strings(treasure: &BTreeMap<String, Treasure>, text_strings: &mut HashMap<String, String>, data_out: &mut Write, header_out: &mut Write) {
    for (name, data) in treasure.iter() {

        add_text(text_strings, name, "name", Some(&data.name));
        //add_text(text_strings, name, "name", Some(name));
        add_text_string(text_strings, name, "desc", &data.desc);
        println!("{:?}", &data.desc);

    }
}




fn enemies_make_strings(enemies: &BTreeMap<String, Enemy>, text_strings: &mut HashMap<String, String>, _data_out: &mut Write, _header_out: &mut Write) {
    for (name, data) in enemies.iter() {
        add_text(text_strings, name, "name", Some(&data.name));
        //add_text(text_strings, name, "name", Some(name));
        add_text_string(text_strings, name, "desc", &data.desc);

    }
}





fn process_enemies(enemies: &BTreeMap<String, Enemy>, _text_strings: &mut HashMap<String, String>, data_out: &mut Write, header_out: &mut Write) {
    println!("Processing Enemies");
    writeln!(data_out, "### Enemy Prefabs ###").unwrap();
    for (name, data) in enemies.iter() {
        let x = 0;
        let y = 0;
        let flags = 0;
        let mut ai = String::from("default");
        if let Some(a) = &data.ai { ai = a.clone();}
        let sprite = data.art.clone();

        write!(data_out, ": enemy_{} {} {} {} tobytes SPR_{} tobytes SPR_portrait_{} tobytes word_{}-desc tobytes ai_{} tobytes word_{}-name", name, x, y, flags, sprite, sprite, name, ai, name);
        writeln!(data_out, " # '{}'", name);
    }
    writeln!(data_out, "0xFF\n### End Enemy Prefabs ###\n\n");

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
    writeln!(data_out, "\n: enemy_ptrs\n");
    for  (name, data) in enemies.iter() {
        writeln!(data_out, "    tobytes enemy_{}", name);
    }

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

fn process_weapons(weapons: &BTreeMap<String, Weapon>, _text_strings: &mut HashMap<String, String>, data_out: &mut Write) {
    println!("Processing Weapons");
    writeln!(data_out, "### Weapon Data ###").unwrap();
    for (_name, _data) in weapons {
        //println!("{} -> {:?}\n", name, data);

    }
    writeln!(data_out, "### End Weapon Data ###\n\n").unwrap();
}




fn read_file(filename: &str) -> String {
    let mut toml_file = File::open(filename).unwrap();
    let mut toml_string = String::new();
    toml_file.read_to_string(&mut toml_string).unwrap();

    return toml_string;
}

fn main() {

    let mut text_strings = HashMap::<String, String>::new();

    //let attacks_string = read_file("../../assets/prefabs/attacks.toml");
    let biomes_string = read_file("../../assets/prefabs/biomes.toml");
    let enemies_string = read_file("../../assets/prefabs/enemies.toml");
    let treasure_string = read_file("../../assets/prefabs/treasure.toml");
    let _weapons_string = read_file("../../assets/prefabs/weapons.toml");

//    let attacks: BTreeMap<String, Attack> = toml::from_str(&attacks_string).unwrap();
/*    println!("\nAttacks:", );
    for atk in &attacks {
        println!("{:?}\n", atk);
    }
*/

    let biomes: BTreeMap<String, Biome> = toml::from_str(&biomes_string).unwrap();
    let enemies: BTreeMap<String, Enemy> = toml::from_str(&enemies_string).unwrap();
    let treasure: BTreeMap<String, Treasure> = toml::from_str(&treasure_string).unwrap();
//    let weapons: BTreeMap<String, Weapon> = toml::from_str(&weapons_string).unwrap();
/*    println!("\nWeapons:", );
    for w in &weapons {
        println!("{:?}\n", w);
    }
*/


    let mut header_dest = File::create("build/prefab_header.o8").unwrap();
    let mut data_dest = File::create("build/prefab_data.o8").unwrap();

    text_strings.insert(String::from("Title_Title"), String::from("Dig Site 8"));
    text_strings.insert(String::from("Title_Credits"), String::from("Credits"));
    text_strings.insert(String::from("Title_Begin"), String::from("Explore the Site"));
    text_strings.insert(String::from("Title_Beastiary"), String::from("View the Beastiary"));
    text_strings.insert(String::from("Title_Itemiary"), String::from("View the Treasure Codex"));
    text_strings.insert(String::from("Title_Soundiary"), String::from("Sound Test"));


    text_strings.insert(String::from("Soundiary_Cool"), String::from("Cool"));
    text_strings.insert(String::from("Soundiary_Octojam"), String::from("Octojam"));
    text_strings.insert(String::from("Soundiary_Five"), String::from("Five"));
    text_strings.insert(String::from("Soundiary_Title"), String::from("Title"));
    text_strings.insert(String::from("Soundiary_PhoneHey"), String::from("PhoneHey"));
    text_strings.insert(String::from("Soundiary_Ruin"), String::from("Ruin"));
    text_strings.insert(String::from("Soundiary_WelcomeBack"), String::from("WelcomeBack"));
    text_strings.insert(String::from("Soundiary_Hello"), String::from("Hello"));


    text_strings.insert(String::from("Credits_1"), String::from("Credits Go Here"));
    text_strings.insert(String::from("New_Level"), String::from("Now Entering"));
    text_strings.insert(String::from("End_Win_Msg"), String::from("You Win!!"));
    text_strings.insert(String::from("Char_Name"), String::from("Name:"));
    text_strings.insert(String::from("Char_Height"), String::from("Height:"));
    text_strings.insert(String::from("Char_Build"), String::from("Build:"));
    text_strings.insert(String::from("Char_Eyes"), String::from("Eyes:"));

    text_strings.insert(String::from("Char_Height_0"), String::from("Average"));
    text_strings.insert(String::from("Char_Height_1"), String::from("Short"));
    text_strings.insert(String::from("Char_Height_2"), String::from("Above Average"));
    text_strings.insert(String::from("Char_Height_3"), String::from("Very Tall"));

    text_strings.insert(String::from("Char_Build_0"), String::from("A Little Extra"));
    text_strings.insert(String::from("Char_Build_1"), String::from("Curvy"));
    text_strings.insert(String::from("Char_Build_2"), String::from("Full Figured"));
    text_strings.insert(String::from("Char_Build_3"), String::from("Skinny"));
    text_strings.insert(String::from("Char_Build_4"), String::from("Average"));
    text_strings.insert(String::from("Char_Build_5"), String::from("Fit"));
    text_strings.insert(String::from("Char_Build_6"), String::from("Jacked"));
    text_strings.insert(String::from("Char_Build_7"), String::from("Rather Not Say"));

    text_strings.insert(String::from("Char_Eyes_0"), String::from("Blue"));
    text_strings.insert(String::from("Char_Eyes_1"), String::from("Brown"));
    text_strings.insert(String::from("Char_Eyes_2"), String::from("Green"));
    text_strings.insert(String::from("Char_Eyes_3"), String::from("Hazel"));
    text_strings.insert(String::from("Char_Eyes_4"), String::from("Black"));
    text_strings.insert(String::from("Char_Eyes_5"), String::from("Grey"));
    text_strings.insert(String::from("Char_Eyes_6"), String::from("Red"));
    text_strings.insert(String::from("Char_Eyes_7"), String::from("Shifty"));

    text_strings.insert(String::from("testthis"), String::from("\"sphinx of black quartz, judge my vow\"? ABCDEF GHIJK LMNOP QRSTU VWXYZ abcd efgh ijkl mnop qrst uvwx yz"));



    //process_weapons(&weapons, &mut text_strings, &mut data_dest);
    //process_attacks(&attacks, &mut text_strings, &mut data_dest);
    enemies_make_strings(&enemies, &mut text_strings, &mut data_dest, &mut header_dest);
    treasure_make_strings(&treasure, &mut text_strings, &mut data_dest, &mut header_dest);
    biomes_make_strings(&biomes, &mut text_strings, &mut data_dest, &mut header_dest);

    process_strings(&text_strings, &mut data_dest, &mut header_dest);
    process_enemies(&enemies, &mut text_strings, &mut data_dest, &mut header_dest);
    process_treasure(&treasure, &mut text_strings, &mut data_dest, &mut header_dest);

//    process_enemies(&enemies, &mut text_strings, &mut data_dest, &mut header_dest);
    process_enemy_lists(&biomes, &mut text_strings, &mut data_dest, &mut header_dest);
    process_treasure_lists(&biomes, &mut text_strings, &mut data_dest, &mut header_dest);

    process_biomes(&biomes, &mut text_strings, &mut data_dest, &mut header_dest);


    writeln!(data_dest, ": entity_table_address tobytes entity_table").unwrap();

}

#[derive(Debug, Clone)]
enum Symbol {
    Letter(char),
    Word(String),
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
