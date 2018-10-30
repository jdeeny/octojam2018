use std::fs::File;
use std::io::Read;
use toml::Value;


fn process_attacks() {

}

fn process_biomes() {

}

fn process_enemies() {

}

fn process_weapons() {

}

fn read_file(filename: &str) -> String {
    let mut toml_file = File::open(filename).unwrap();
    let mut toml_string = String::new();
    toml_file.read_to_string(&mut toml_string).unwrap();

    return toml_string;
}

fn main() {

    let attacks_string = read_file("../../assets/prefabs/attacks.toml");
    let biomes_string = read_file("../../assets/prefabs/biomes.toml");
    let enemies_string = read_file("../../assets/prefabs/enemies.toml");
    let treasure_string = read_file("../../assets/prefabs/treasure.toml");
    let weapons_string = read_file("../../assets/prefabs/weapons.toml");

    let attacks = attacks_string.parse::<Value>().unwrap();
    let biomes = biomes_string.parse::<Value>().unwrap();
    let enemies = enemies_string.parse::<Value>().unwrap();
    let treasure = treasure_string.parse::<Value>().unwrap();
    let weapons = weapons_string.parse::<Value>().unwrap();

    println!("{:?}\n", attacks);
    println!("{:?}\n", biomes);
    println!("{:?}\n", enemies);
    println!("{:?}\n", treasure);
    println!("{:?}\n", weapons);

}
