const FOLDERS: &[(&'static str, &'static str, bool); 2] = &[
    ("src/resources/asm_scholar/", "scholar", false),
    ("src/resources/asm_vanilla/", "vanilla", true),
];

fn main() {
    shared::object::build(FOLDERS)
}