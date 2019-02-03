use std::collections::BTreeMap;
use std::io::Write;
use toml::Value;



#[derive(Debug, Deserialize)]
struct Weapon {
    Type: Option<String>,
    Damage: Option<String>,
    AttackDescription: Option<Vec<String>>,

}

pub struct Weapons {
    weapons: BTreeMap<String, Weapon>,
}

impl Weapons {
    pub fn from_toml(toml_str: &str) -> Self {
        let weapons: BTreeMap<String, Weapon> = toml::from_str(toml_str).unwrap();
        return Self { weapons: weapons };
    }

    pub fn header(&self, out: &Write) {

    }

    pub fn data(&self, out: &Write) {

    }


    pub fn process(&mut self) {

    }

}



/*

fn process_weapons(weapons: &BTreeMap<String, Weapon>, _text_strings: &mut BtreeMap<String, String>, data_out: &mut Write) {
    println!("Processing Weapons");
    writeln!(data_out, "### Weapon Data ###").unwrap();
    for (_name, _data) in weapons {
        //println!("{} -> {:?}\n", name, data);

    }
    writeln!(data_out, "### End Weapon Data ###\n\n").unwrap();
}
*/
