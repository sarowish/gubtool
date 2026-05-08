use bincode;
use object::{Object, ObjectSection, ObjectSymbol, RelocationTarget};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

#[derive(Serialize, Deserialize, Debug)]
pub struct AsmFolder {
    pub functions: Vec<AsmFunction>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AsmFunction {
    pub name: String,
    pub bytes: Vec<u8>,
    pub relocations: Vec<Relocation>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relocation {
    pub symbol: String,
    pub offset: u64,
}

const FOLDERS: [(&'static str, &'static str, bool); 3] = [
    ("src/ds2/resources/asm_scholar/", "scholar", false),
    ("src/ds2/resources/asm_vanilla/", "vanilla", true),
    ("src/er/resources/asm/", "eldenring", false),
];

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    for (path, name, is_32) in FOLDERS {
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
            if is_32 {
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
                        relocations.push(Relocation {
                            symbol: symbol.name().unwrap().to_string(),
                            offset,
                        });
                    }
                    _ => (),
                }
            }

            functions.push(AsmFunction {
                name: file_stem.to_string(),
                bytes: text.to_vec(),
                relocations,
            });
        }

        let folder = AsmFolder { functions };
        let encoded = bincode::serialize(&folder).unwrap();
        let out_file = out_dir.join(format!("{name}.bin"));
        fs::write(&out_file, encoded).unwrap();

        println!("cargo:rerun-if-changed={}", &path);
    }
}
