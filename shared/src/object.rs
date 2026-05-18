use object::{Object, ObjectSection, ObjectSymbol, RelocationTarget};
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf, process::Command};

#[derive(Serialize, Deserialize, Debug)]
pub struct AsmFolder {
    functions: Vec<AsmFunction>,
}

impl AsmFolder {
    pub fn new(functions: Vec<AsmFunction>) -> Self {
        Self { functions }
    }
    pub fn get_function(&self, name: &'static str) -> &AsmFunction {
        self.functions.iter().find(|s| s.name == name).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AsmFunction {
    name: String,
    bytes: Vec<u8>,
    relocations: Vec<Relocation>,
}

impl AsmFunction {
    pub fn new(name: String, bytes: Vec<u8>, relocations: Vec<Relocation>) -> Self {
        Self { name, bytes, relocations }
    }
    pub fn get_bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }
    pub fn reloc(&self, name: &'static str) -> u64 {
        self.relocations.iter().find(|s| s.symbol == name).unwrap().offset
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relocation {
    symbol: String,
    offset: u64,
}

impl Relocation {
    pub fn new(symbol: String, offset: u64) -> Self {
        Self { symbol, offset }
    }
}

pub fn build(folders: &[(&'static str, &'static str, bool)]) {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    for (path, name, is_32) in folders {
        let folder_out = out_dir.join(name);
        fs::create_dir_all(&folder_out).unwrap();

        let mut functions = Vec::<AsmFunction>::new();

        for file in fs::read_dir(path).unwrap() {

            // compile

            let file_path = file.unwrap().path();

            let file_stem = file_path.file_stem().unwrap().to_string_lossy();
            let obj = folder_out.join(format!("{file_stem}.o"));

            let mut cmd = Command::new("cc");
            cmd.arg("-c");
            if *is_32 {
                cmd.arg("-m32");
            }
            cmd.arg(&file_path);
            cmd.arg("-o");
            cmd.arg(&obj);
            let status = cmd.status().unwrap();
            assert!(status.success(), "failed to compile {:?}", file_path);

            println!("cargo:rerun-if-changed={}", &file_path.display());

            // parse object file

            let mut relocations: Vec<Relocation> = Vec::new();

            let bytes = fs::read(&obj).unwrap();
            let obj_file = object::File::parse(&*bytes).unwrap();

            let section = obj_file.section_by_name(".text").unwrap();
            let text = section.data().unwrap();

            for (offset, reloc) in section.relocations() {
                match reloc.target() {
                    RelocationTarget::Symbol(symbol_index) => {
                        let symbol = obj_file.symbol_by_index(symbol_index).unwrap();
                        relocations
                            .push(Relocation::new(symbol.name().unwrap().to_string(), offset));
                    }
                    _ => (),
                }
            }

            functions.push(AsmFunction::new(
                file_stem.to_string(),
                text.to_vec(),
                relocations,
            ));
        }

        let folder = AsmFolder::new(functions);
        let encoded = bincode::serialize(&folder).unwrap();
        let out_file = out_dir.join(format!("{name}.bin"));
        fs::write(&out_file, encoded).unwrap();

        println!("cargo:rerun-if-changed={}", &path);
    }
}