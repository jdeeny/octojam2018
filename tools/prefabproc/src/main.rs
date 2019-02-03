use std::collections::HashMap;
use std::fs::File;
use std::io::{Read,Write};

#[macro_use]
extern crate serde_derive;


mod font;
mod biomes;
mod enemies;
mod treasure;
mod weapons;


fn read_file(filename: &str) -> String {
    let mut toml_file = File::open(filename).unwrap();
    let mut toml_string = String::new();
    toml_file.read_to_string(&mut toml_string).unwrap();

    return toml_string;
}

fn main() {
    let mut header_dest = File::create("build/prefab_header.o8").unwrap();
    let mut data_dest = File::create("build/prefab_data.o8").unwrap();

    let mut text_strings = HashMap::<String, String>::new();

    let biomes_string = read_file("../../assets/prefabs/biomes.toml");
    let enemies_string = read_file("../../assets/prefabs/enemies.toml");
    let treasure_string = read_file("../../assets/prefabs/treasure.toml");
    let weapons_string = read_file("../../assets/prefabs/weapons.toml");
    let font_string = read_file("../../assets/prefabs/font.toml");
    let _special_strings_string = read_file("../../assets/prefabs/strings.toml");

    let font = font::Font::from_toml(&font_string);
    let biomes = biomes::Biomes::from_toml(&biomes_string);
    let enemies = enemies::Enemies::from_toml(&enemies_string);
    let treasure = treasure::Treasure::from_toml(&treasure_string);
    let weapons = weapons::Weapons::from_toml(&weapons_string);


/*
    let font: BTreeMap<String, String> = toml::from_str(&font_string).unwrap();
    let biomes: BTreeMap<String, Biome> = toml::from_str(&biomes_string).unwrap();
    let enemies: BTreeMap<String, Enemy> = toml::from_str(&enemies_string).unwrap();
    let treasure: BTreeMap<String, Treasure> = toml::from_str(&treasure_string).unwrap();
    let special_strings: BTreeMap<String, String> = toml::from_str(&special_strings_string).unwrap();



    process_font(&font, &mut data_dest, &mut header_dest);

    for (name, s) in special_strings { text_strings.insert(name, s); }

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

*/
    writeln!(data_dest, ": entity_table_address tobytes entity_table").unwrap();

}
