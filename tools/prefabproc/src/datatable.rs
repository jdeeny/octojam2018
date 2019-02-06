use enum_kinds::*;
#[derive(EnumKind)]
#[enum_kind(DataKind)]
pub enum Data {
    Byte(u8),
    Label(String),
}

struct Row {
    name: String,
    data: Vec<Data>,
}

pub struct DataTable {
    cols: Vec<(String, DataKind)>,
    entries: Vec<Row>
}

impl DataKind {
        pub fn bytes(&self) -> usize {
            return match self {
                DataKind::Byte => 1,
                DataKind::Label => 2,
            };
        }
}

impl DataTable {
    pub fn new(cols: &Vec<(String, DataKind)>) -> Self {
        return DataTable { cols: cols.clone(), entries: Vec::new() }
    }

    pub fn row_bytes(&self) -> usize {
        let mut total = 0;
        for (_, kind) in &self.cols {
            match kind {
                DataKind::Byte => { total += 1; },
                DataKind::Label => { total += 2; },
            }
        }
        return total;
    }

    pub fn len(&self) -> usize {
        return self.entries.len();
    }

    pub fn data_bytes(&self) -> usize {
        return self.row_bytes() * self.len();
    }

    pub fn octo_data(&self) -> String {
        return String::from("");
    }

    pub fn octo_header(&self) -> String {
        /*writeln!(out, ":const BIOME.NAME 0").unwrap();
        writeln!(out, ":const BIOME.NARRATION 2").unwrap();
        writeln!(out, ":const BIOME.ENEMYSET 4").unwrap();
        writeln!(out, ":const BIOME.TREASURESET 6").unwrap();*/
        return String::from("");
    }
}
