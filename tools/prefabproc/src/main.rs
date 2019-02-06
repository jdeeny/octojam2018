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
    let mut code_dest = File::create("build/prefab_code.o8").unwrap();

    let font_string = read_file("../../assets/prefabs/font.toml");
    let mut font = font::Font::from_toml(&font_string);
    // Mark some glyphs used so we are sure they can be used in code
    font.mark_used("01234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz");

    let biomes_string = read_file("../../assets/prefabs/biomes.toml");
    let biomes = biomes::Biomes::from_toml(&biomes_string);
    let enemies_string = read_file("../../assets/prefabs/enemies.toml");
    let enemies = enemies::Enemies::from_toml(&enemies_string);
    let treasure_string = read_file("../../assets/prefabs/treasure.toml");
    let treasure = treasure::Treasure::from_toml(&treasure_string);

    let special_strings_string = read_file("../../assets/prefabs/strings.toml");
    let mut dict = text::Dictionary::new(font);
    dict.insert_from_toml(&special_strings_string);
    enemies.process_strings(&mut dict);
    treasure.process_strings(&mut dict);
    biomes.process_strings(&mut dict);
    dict.process();

    dict.header(&mut header_dest);
    enemies.header(&mut header_dest);
    biomes.header(&mut header_dest);
    treasure.header(&mut header_dest);

    dict.data(&mut data_dest);
    enemies.data(&mut data_dest);
    biomes.data(&mut data_dest);
    treasure.data(&mut data_dest);

    writeln!(data_dest, ": entity_table_address tobytes entity_table").unwrap();

    dict.code(&mut code_dest);
    enemies.code(&mut code_dest);
    biomes.code(&mut code_dest);
    treasure.code(&mut code_dest);
}
