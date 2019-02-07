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
    name: String,
    cols: Vec<(String, DataKind)>,
    entries: Vec<Row>
}

impl Data {
    pub fn to_string(&self) -> String {
        return match self {
            Data::Byte(b) => format!(":byte {}", b),
            Data::Label(l) => format!("TB {}", l.replace(" ", "_")),
        }
    }
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
    pub fn new(name: &str, cols: &Vec<(String, DataKind)>) -> Self {
        return DataTable { name: name.into(), cols: cols.clone(), entries: Vec::new() }
    }

    pub fn add(&mut self, name: &str, entry: Vec<Data>) {
        assert!(entry.len() == self.cols.len(), "Entry length ({}) must equal number of columns ({})", entry.len(), self.cols.len());
        self.entries.push(Row { name: name.into(), data: entry });
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

    pub fn bytes(&self) -> usize {
        return self.row_bytes() * self.len();
    }

    pub fn octo_data(&self) -> String {
        let mut s = format!("## {} Data Table  - {} entries x {} bytes = {} bytes\n",
        &self.name, self.len(), self.row_bytes(), self.bytes());

        s += "#  | ";
        for (name, col) in &self.cols {
            s += &format!("{} | ", name );
        }
        s += "\n";
        for row in &self.entries {
            s += &format!(": {} ", &row.name);
            for col in &row.data {
                s += &format!("{} ", &col.to_string());
            }
            s += "\n";
        }
        return s;
    }

    pub fn octo_code(&self) -> String {
        return String::from("");
    }

    pub fn octo_header(&self) -> String {
        let mut s = format!(":const {}_COUNT {}\n:const {}_LAST {}\n:const {}_ROWBYTES {}\n",
            self.name.to_uppercase(), self.cols.len(), self.name.to_uppercase(), self.cols.len() - 1,
            self.name.to_uppercase(), self.row_bytes());
        let mut offset = 0;
        for (name, col) in &self.cols {
            s += &format!(":const {}.{} {:3}\n", self.name.to_uppercase(), name.to_uppercase(), offset );
            offset += col.bytes();
        }
        return s;
    }
}
