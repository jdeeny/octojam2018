
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
    tileset: Option<String>,
    levels: Option<usize>,
    enemies: Option<Vec<String>>,
    treasure: Option<Vec<String>>,
    narration: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Enemy {
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
    name: Option<String>,
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

    writeln!(data_out, "### Attack Data ###");
    for (name, data) in attacks.iter() {
        if let Some(hitmsg) = &data.Hit {
            add_text(text_strings, name, "hitmsg", Some(hitmsg));
        }
        if let Some(missmsg) = &data.Miss {
            add_text(text_strings, name, "missmsg", Some(missmsg));
        }
    }
    writeln!(data_out, "### End Attack Data ###\n\n");


}

fn process_biomes(biomes: &BTreeMap<String, Biome>, text_strings: &mut HashMap<String, String>, data_out: &mut Write) {
    println!("Processing Biomes");
    for (name, data) in biomes.iter() {
//        println!("{} -> {:?}", &name, data);
        if let Some(narration) = &data.narration {
            let mut key: String = name.clone();
            key.push_str("-narration");
            text_strings.insert(key, narration.clone());
        }
    }

    writeln!(data_out, "# Biomes (Level List)");
    writeln!(data_out, ": biome_state tobytes HERE 0");
    for (name, data) in biomes.iter() {
        let mut lvls = 1;
        if let Some(levels) = data.levels { lvls = levels; }

        // tileset - unused for now
        // create a word for the name
        // create an enemy set
        // create a narration event
        // create a treasure set

        writeln!(data_out, "# '{}', {} levels", name, lvls);
        writeln!(data_out, ": biome_{}      :byte {}       tobytes word_{}     tobytes narration_{}     tobytes enemyset_{}\n", lvls, name, name, name, name);
    }
    writeln!(data_out, "0xFF # End biomes\n\n");
    writeln!(data_out, "### End Biome Data ###\n\n");

}

fn process_enemies(enemies: &BTreeMap<String, Enemy>, text_strings: &mut HashMap<String, String>, data_out: &mut Write, header_out: &mut Write) {
    println!("Processing Enemies");
    writeln!(data_out, "### Enemy Prefabs ###");
    for (name, data) in enemies.iter() {
        let x = 0;
        let y = 0;
        let flags = 0;
        let mut ai = String::from("default");
        if let Some(a) = &data.ai { ai = a.clone();}
        let sprite = data.art.clone();

        add_text(text_strings, name, "name", Some(name));
        add_text_string(text_strings, name, "desc", &data.desc);

        write!(data_out, ": enemy_{} {} {} {} tobytes SPR_{} tobytes ai_{}", name, x, y, flags, sprite, ai);
        writeln!(data_out, " # '{}'", name);
    }
    writeln!(data_out, "0xFF\n### End Enemy Prefabs ###\n\n");

    writeln!(header_out, ":const enemy_prefab_count {}", enemies.keys().count());
}

fn process_treasure(treasure: &BTreeMap<String, Treasure>, text_strings: &mut HashMap<String, String>, data_out: &mut Write, header_out: &mut Write) {
    println!("Processing Treasure");
    writeln!(data_out, "### Treasure Prefabs ###");
    for (name, data) in treasure.iter() {
        let x = 0;
        let y = 0;
        let flags = 0;
        let ai = String::from("treasure");
        let sprite = data.art.clone();

        add_text(text_strings, name, "name", Some(name));
        add_text_string(text_strings, name, "desc", &data.desc);


        write!(data_out, ": treasure_{} {} {} {} tobytes SPR_{} tobytes ai_{}", name, x, y, flags, sprite, ai);
        writeln!(data_out, " # '{}'", name);
    }
    writeln!(data_out, "0xFF\n### End Treasure Data ###\n\n");

    writeln!(header_out, ":const treasure_prefab_count {}", treasure.keys().count());



}

fn process_weapons(weapons: &BTreeMap<String, Weapon>, text_strings: &mut HashMap<String, String>, data_out: &mut Write) {
    println!("Processing Weapons");
    writeln!(data_out, "### Weapon Data ###");
    for (name, data) in weapons {
        //println!("{} -> {:?}\n", name, data);

    }
    writeln!(data_out, "### End Weapon Data ###\n\n");
}




fn read_file(filename: &str) -> String {
    let mut toml_file = File::open(filename).unwrap();
    let mut toml_string = String::new();
    toml_file.read_to_string(&mut toml_string).unwrap();

    return toml_string;
}

fn main() {

    let mut text_strings = HashMap::<String, String>::new();

    let attacks_string = read_file("../../assets/prefabs/attacks.toml");
    let biomes_string = read_file("../../assets/prefabs/biomes.toml");
    let enemies_string = read_file("../../assets/prefabs/enemies.toml");
    let treasure_string = read_file("../../assets/prefabs/treasure.toml");
    let weapons_string = read_file("../../assets/prefabs/weapons.toml");

    let attacks: BTreeMap<String, Attack> = toml::from_str(&attacks_string).unwrap();
/*    println!("\nAttacks:", );
    for atk in &attacks {
        println!("{:?}\n", atk);
    }
*/

    let biomes: BTreeMap<String, Biome> = toml::from_str(&biomes_string).unwrap();
/*    println!("\nBiomes:", );
    for b in &biomes {
        println!("{:?}\n", b);
    }
*/

    let enemies: BTreeMap<String, Enemy> = toml::from_str(&enemies_string).unwrap();
/*    println!("\nEnemies:", );
    for e in &enemies {
        println!("{:?}\n", e);
    }
*/
    let treasure: BTreeMap<String, Treasure> = toml::from_str(&treasure_string).unwrap();
/*    println!("\nTreasure:", );
    for t in &treasure {
        println!("{:?}\n", t);
    }
*/

    let weapons: BTreeMap<String, Weapon> = toml::from_str(&weapons_string).unwrap();
/*    println!("\nWeapons:", );
    for w in &weapons {
        println!("{:?}\n", w);
    }
*/


    let mut header_dest = File::create("build/prefab_header.o8").unwrap();
    let mut data_dest = File::create("build/prefab_data.o8").unwrap();

    //process_biomes(&biomes, &mut text_strings, &mut data_dest);
    //process_weapons(&weapons, &mut text_strings, &mut data_dest);
    //process_attacks(&attacks, &mut text_strings, &mut data_dest);
    process_enemies(&enemies, &mut text_strings, &mut data_dest, &mut header_dest);
    process_treasure(&treasure, &mut text_strings, &mut data_dest, &mut header_dest);

    writeln!(data_dest, ": entity_table_address tobytes entity_table");
    println!("Strings:");
    for (k, v) in text_strings {
        print!("{} -> \"{}\"\n\n", k, v);
    }
}

enum Symbol {
    Letter(char),
    Replacement(usize),
    Whitespace,
}
