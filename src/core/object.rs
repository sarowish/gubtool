use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AsmFolder {
    pub functions: Vec<AsmFunction>,
}

impl AsmFolder {
    pub fn get_function(&self, name: &'static str) -> &AsmFunction {
        self.functions.iter().find(|s| s.name == name).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AsmFunction {
    pub name: String,
    pub bytes: Vec<u8>,
    pub relocations: Vec<Relocation>,
}

impl AsmFunction {
    pub fn reloc(&self, name: &'static str) -> u64 {
        self.relocations.iter().find(|s| s.symbol == name).unwrap().offset
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relocation {
    pub symbol: String,
    pub offset: u64,
}