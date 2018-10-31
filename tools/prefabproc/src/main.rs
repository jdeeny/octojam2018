use std::collections::{ HashMap, BTreeMap };
use std::fs::File;
use std::io::Read;
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


fn process_attacks(attacks: &Value, text_strings: &mut HashMap<String, String>) {
    println!("{:?}\n", attacks);

    /*if let Value::Table(atk_table) = attacks {
        let decoded: Attack = toml::from_str(toml_str).unwrap();
        for (name, data) in atk_table.iter() {
            println!("{} -> {:?}\n", name, data);
        }

    }*/
}

fn process_biomes(biomes: &Value, text_strings: &mut HashMap<String, String>) {
    println!("{:?}\n", biomes);

}

fn process_enemies(enemies: &Value, text_strings: &mut HashMap<String, String>) {
    println!("{:?}\n", enemies);

}

fn process_treasure(treasure: &Value, text_strings: &mut HashMap<String, String>) {
    println!("{:?}\n", treasure);
}

fn process_weapons(weapons: &Value, text_strings: &mut HashMap<String, String>) {
    println!("{:?}\n", weapons);

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
    for atk in attacks {
        println!("{:?}\n", atk);
    }

    println!("\nBiomes:", );
    let biomes: BTreeMap<String, Biome> = toml::from_str(&biomes_string).unwrap();
    for b in biomes {
        println!("{:?}\n", b);
    }

    println!("\nEnemies:", );
    let enemies: BTreeMap<String, Enemy> = toml::from_str(&enemies_string).unwrap();
    for e in enemies {
        println!("{:?}\n", e);
    }

    println!("\nTreasure:", );
    let treasure: BTreeMap<String, Treasure> = toml::from_str(&treasure_string).unwrap();
    for t in treasure {
        println!("{:?}\n", t);
    }


    let weapons: BTreeMap<String, Weapon> = toml::from_str(&weapons_string).unwrap();
    println!("\nWeapons:", );
    for w in weapons {
        println!("{:?}\n", w);
    }


/*
    let attacks = attacks_string.parse::<Value>().unwrap();
    let biomes = biomes_string.parse::<Value>().unwrap();
    let enemies = enemies_string.parse::<Value>().unwrap();
    let treasure = treasure_string.parse::<Value>().unwrap();
    let weapons = weapons_string.parse::<Value>().unwrap();

    process_attacks(&attacks, &mut text_strings);
    process_biomes(&biomes, &mut text_strings);
    process_enemies(&enemies, &mut text_strings);
    process_treasure(&treasure, &mut text_strings);
    process_weapons(&weapons, &mut text_strings);
*/

}
