use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read,Write};

#[macro_use]
extern crate serde_derive;


mod font;
mod biomes;
mod enemies;
mod treasure;
mod text;


fn read_file(filename: &str) -> String {
    let mut toml_file = File::open(filename).unwrap();
    let mut toml_string = String::new();
    toml_file.read_to_string(&mut toml_string).unwrap();

    return toml_string;
}

fn main() {
    let mut header_dest = File::create("build/prefab_header.o8").unwrap();
    let mut data_dest = File::create("build/prefab_data.o8").unwrap();

    let biomes_string = read_file("../../assets/prefabs/biomes.toml");
    let enemies_string = read_file("../../assets/prefabs/enemies.toml");
    let treasure_string = read_file("../../assets/prefabs/treasure.toml");
    let font_string = read_file("../../assets/prefabs/font.toml");
    let special_strings_string = read_file("../../assets/prefabs/strings.toml");

    let font = font::Font::from_toml(&font_string);
    let biomes = biomes::Biomes::from_toml(&biomes_string);
    let enemies = enemies::Enemies::from_toml(&enemies_string);
    let treasure = treasure::Treasure::from_toml(&treasure_string);
    let special_strings: BTreeMap<String, String> = toml::from_str(&special_strings_string).unwrap();

    let mut dict = text::Dictionary::new();

    for (name, s) in special_strings { dict.insert(&name, &s); }

    enemies.process_strings(&mut dict);
    treasure.process_strings(&mut dict);
    biomes.process_strings(&mut dict);

    dict.process();

    font.header(&mut header_dest);
    dict.header(&mut header_dest);
    enemies.header(&mut header_dest);
    treasure.header(&mut header_dest);

    font.data(&mut data_dest);
    dict.data(&mut data_dest);
    enemies.data(&mut data_dest);
    treasure.data(&mut data_dest);



/*
    process_strings(&text_strings, &mut data_dest, &mut header_dest);
    process_enemies(&enemies, &mut text_strings, &mut data_dest, &mut header_dest);
    process_treasure(&treasure, &mut text_strings, &mut data_dest, &mut header_dest);

    process_enemy_lists(&biomes, &mut text_strings, &mut data_dest, &mut header_dest);
    process_treasure_lists(&biomes, &mut text_strings, &mut data_dest, &mut header_dest);

    process_biomes(&biomes, &mut text_strings, &mut data_dest, &mut header_dest);

*/
    writeln!(data_dest, ": entity_table_address tobytes entity_table").unwrap();

}
