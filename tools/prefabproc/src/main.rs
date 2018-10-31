
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
    art: Option<String>,
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
    art: Option<String>,
    active: Option<Vec<Value>>,
    desc: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Weapon {
    Type: Option<String>,
    Damage: Option<String>,
    AttackDescription: Option<Vec<String>>,

}


fn process_attacks(attacks: &BTreeMap<String, Attack>, text_strings: &mut HashMap<String, String>, data_out: &mut Write) {

    for (name, data) in attacks.iter() {
        println!("{} -> {:?}", &name, data);
        if let Some(hitmsg) = &data.Hit {
            let mut key: String = name.clone();
            key.push_str("-hitmsg");
            text_strings.insert(key, hitmsg.clone());
        }
        if let Some(missmsg) = &data.Miss {
            let mut key: String = name.clone();
            key.push_str("-missmsg");
            text_strings.insert(key, missmsg.clone());
        }
    }
}

fn process_biomes(biomes: &BTreeMap<String, Biome>, text_strings: &mut HashMap<String, String>, data_out: &mut Write) {
    for (name, data) in biomes.iter() {
        println!("{} -> {:?}", &name, data);
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
        writeln!(data_out, "0xFF # End biomes\n\n");
    }

}

fn process_enemies(enemies: &BTreeMap<String, Enemy>, text_strings: &mut HashMap<String, String>, data_out: &mut Write) {
    println!("{:?}\n", enemies);

}

fn process_treasure(treasure: &BTreeMap<String, Treasure>, text_strings: &mut HashMap<String, String>, data_out: &mut Write) {
    println!("{:?}\n", treasure);
}

fn process_weapons(weapons: &BTreeMap<String, Weapon>, text_strings: &mut HashMap<String, String>, data_out: &mut Write) {
    println!("Process Weapons");
    for (name, data) in weapons {
        println!("{} -> {:?}\n", name, data);

    }


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

    println!("\nAttacks:", );
    let attacks: BTreeMap<String, Attack> = toml::from_str(&attacks_string).unwrap();
    for atk in &attacks {
        println!("{:?}\n", atk);
    }

    println!("\nBiomes:", );
    let biomes: BTreeMap<String, Biome> = toml::from_str(&biomes_string).unwrap();
    for b in &biomes {
        println!("{:?}\n", b);
    }

    println!("\nEnemies:", );
    let enemies: BTreeMap<String, Enemy> = toml::from_str(&enemies_string).unwrap();
    for e in &enemies {
        println!("{:?}\n", e);
    }

    println!("\nTreasure:", );
    let treasure: BTreeMap<String, Treasure> = toml::from_str(&treasure_string).unwrap();
    for t in &treasure {
        println!("{:?}\n", t);
    }

    let weapons: BTreeMap<String, Weapon> = toml::from_str(&weapons_string).unwrap();
    println!("\nWeapons:", );
    for w in &weapons {
        println!("{:?}\n", w);
    }

    let mut data_dest = File::create("build/sprite_data.o8").unwrap();


    //process_attacks(&attacks, &mut text_strings, &mut data_dest);
    process_biomes(&biomes, &mut text_strings, &mut data_dest);
    process_weapons(&weapons, &mut text_strings, &mut data_dest);
    process_treasure(&treasure, &mut text_strings, &mut data_dest);
    process_enemies(&enemies, &mut text_strings, &mut data_dest);
    process_attacks(&attacks, &mut text_strings, &mut data_dest);


    /*println!("Strings:");
    for (k, v) in text_strings {
        print!("{} -> \"{}\"\n\n", k, v);
    }*/
}

enum Symbol {
    Letter(char),
    Replacement(usize),
    Whitespace,
}
