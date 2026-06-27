use object::{Object, ObjectSection, ObjectSymbol, RelocationTarget};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, VecDeque},
    env, fs,
    path::PathBuf,
    process::Command,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct AsmFolder {
    pub functions: HashMap<String, AsmFunction>,
}

impl AsmFolder {
    pub fn new(functions: Vec<AsmFunction>) -> Self {
        let map: HashMap<String, AsmFunction> = functions
            .into_iter()
            .map(|fun| (fun.name.clone(), fun))
            .collect();
        Self { functions: map }
    }

    pub fn get_function(&self, name: &'static str) -> AsmFunction {
        self.functions.get(name).unwrap().clone()
    }

    pub fn print_function_sizes(&self) {
        self.functions
            .iter()
            .for_each(|(key, fun)| println!("{}, {:#X}", key, fun.bytes.len()));
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AsmFunction {
    name: String,
    bytes: Vec<u8>,
    relocations: VecDeque<Relocation>,
}

impl AsmFunction {
    pub fn new(name: String, bytes: Vec<u8>, relocations: VecDeque<Relocation>) -> Self {
        Self {
            name,
            bytes,
            relocations,
        }
    }

    pub fn take_bytes(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.bytes)
    }

    pub fn print_relocs(&self) {
        self.relocations
            .iter()
            .for_each(|r| println!("{:#X}, {}", r.offset, r.symbol));
    }

    #[track_caller]
    pub fn reloc(&mut self, name: &'static str) -> u64 {
        let reloc = self.relocations.pop_front().unwrap();

        if reloc.symbol == name {
            reloc.offset
        } else {
            panic!("symbol mismatch")
        }
    }

    #[track_caller]
    pub fn reloc_find(&mut self, name: &'static str) -> u64 {
        let pos = self.relocations
            .iter()
            .position(|s| s.symbol == name)
            .unwrap();
        let popped = self.relocations.remove(pos).unwrap();
        popped.offset
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
            // assemble

            let file_path = file.unwrap().path();

            let file_stem = file_path.file_stem().unwrap().to_string_lossy();
            let obj = folder_out.join(format!("{file_stem}.o"));

            let mut cmd = Command::new("cc");
            cmd.arg("-c");
            if *is_32 {
                cmd.arg("-m32");
            }
            cmd.arg(&file_path);
            cmd.arg("-Wa,-msyntax=intel");
            cmd.arg("-Wa,-mnaked-reg");
            cmd.arg("-o");
            cmd.arg(&obj);
            let status = cmd.status().unwrap();
            assert!(status.success(), "failed to assemble {:?}", file_path);

            println!("cargo:rerun-if-changed={}", &file_path.display());

            // parse object file

            let mut relocations: VecDeque<Relocation> = VecDeque::new();

            let bytes = fs::read(&obj).unwrap();
            let obj_file = object::File::parse(&*bytes).unwrap();

            let section = obj_file.section_by_name(".text").unwrap();
            let text = section.data().unwrap();

            for (offset, reloc) in section.relocations() {
                match reloc.target() {
                    RelocationTarget::Symbol(symbol_index) => {
                        let symbol = obj_file.symbol_by_index(symbol_index).unwrap();
                        relocations
                            .push_back(Relocation::new(symbol.name().unwrap().to_string(), offset));
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